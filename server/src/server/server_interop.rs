use std::{
    hash::{Hash, Hasher},
    net::IpAddr,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;

use tokio::sync::{mpsc, watch};
use tokio::{runtime::Runtime, sync::oneshot};

use crate::{gui::Message, server::server::start_server, utils::await_option};

static ID: AtomicU32 = AtomicU32::new(0);

pub struct Server {
    status: watch::Receiver<ServerStatus>,
    tx: mpsc::UnboundedSender<ServerCommand>,
    runtime: Arc<Runtime>,
}

#[derive(Debug, Clone, Copy)]
pub enum ServerCommand {
    SwitchPort { port: u16 },
    Restart,
    Stop,
    Join,
}

#[derive(Debug, Clone, Copy)]
pub enum ServerStatus {
    Online { ip: IpAddr },
    OnlineNoIp,
    Restarting,
    Offline,
    Err,
}

impl Server {
    pub fn new(runtime: Arc<Runtime>) -> Server {
        let (tx, server_rx) = mpsc::unbounded_channel();
        let (server_tx, rx) = watch::channel(ServerStatus::Offline);

        runtime.spawn(run_server(runtime.clone(), server_rx, server_tx));

        Server {
            status: rx,
            tx,
            runtime,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::from_recipe(ServerSubscription {
            id: ID.fetch_add(1, Ordering::Relaxed),
            status: self.status.clone(),
        })
    }

    pub fn send(&self, command: ServerCommand) {
        self.tx.send(command).ok();
    }
}

struct ServerSubscription {
    id: u32,
    status: watch::Receiver<ServerStatus>,
}

impl<H: Hasher, I> Recipe<H, I> for ServerSubscription {
    type Output = Message;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.id.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(iced_futures::futures::stream::unfold(
            self,
            move |mut server| async move {
                if let Err(_) = server.status.changed().await {
                    return None;
                }

                let status = *server.status.borrow();

                Some((Message::ServerStatus(status), server))
            },
        ))
    }
}

async fn run_server(
    runtime: Arc<Runtime>,
    mut rx: mpsc::UnboundedReceiver<ServerCommand>,
    tx: watch::Sender<ServerStatus>,
) {
    println!("Start server");
    tx.send(ServerStatus::Offline).ok();

    let mut port: u16 = 8000;

    let mut server_canceller: Option<oneshot::Sender<()>> = None;

    let mut status_receiver: Option<mpsc::UnboundedReceiver<ServerStatus>> = None;

    loop {
        tokio::select! {
            maybe_command = rx.recv() => {
                let command = match maybe_command {
                    Some(v) => v,
                    None => break,
                };

                match command {
                    ServerCommand::SwitchPort { port: new_port } => {
                        port = new_port;
                    }
                    ServerCommand::Restart => {
                        tx.send(ServerStatus::Restarting).ok();

                        if let Some(canceller) = server_canceller {
                            canceller.send(()).ok();
                        }

                        let (canceller, cancel_signal) = oneshot::channel();

                        server_canceller = Some(canceller);

                        let (status_tx, status_rx) = mpsc::unbounded_channel();

                        status_receiver = Some(status_rx);

                        runtime.spawn(start_server(port, cancel_signal, status_tx));
                    }
                    ServerCommand::Stop => {
                        if let Some(canceller) = server_canceller {
                            canceller.send(()).ok();
                            server_canceller = None;
                        }
                    }
                    ServerCommand::Join => break,
                }
            }

        Some(maybe_status) = await_option(status_receiver.as_mut().map(|v| v.recv())) => {
                let status = match maybe_status {
                    Some(v) => v,
                    None => break,
                };

                tx.send(status).ok();
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.tx.send(ServerCommand::Join).ok();
    }
}
