use std::{
    convert::Infallible,
    hash::{Hash, Hasher},
    net::{IpAddr, SocketAddr},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use hyper::{server::Server as HttpServer, service, Body, Request, Response};
use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;

use tokio::sync::{mpsc, watch};
use tokio::{runtime::Runtime, sync::oneshot};

use crate::{
    gui::Message,
    utils::{await_option, get_local_ip},
};

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
                            canceller.send(());
                        }

                        let (canceller, cancel_signal) = oneshot::channel();

                        server_canceller = Some(canceller);

                        let (status_tx, status_rx) = mpsc::unbounded_channel();

                        status_receiver = Some(status_rx);

                        runtime.spawn(start_server(port, cancel_signal, status_tx));
                    }
                    ServerCommand::Stop => {
                        if let Some(canceller) = server_canceller {
                            canceller.send(());
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

                tx.send(status);
            }
        }
    }
}

async fn start_server(
    port: u16,
    cancel_signal: oneshot::Receiver<()>,
    status_sender: mpsc::UnboundedSender<ServerStatus>,
) {
    println!("Starting server");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // And a MakeService to handle each connection...
    let make_service = service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service::service_fn(handle))
    });

    // Then bind and serve...
    let server = HttpServer::bind(&addr).serve(make_service);

    let graceful = server.with_graceful_shutdown(async {
        cancel_signal.await.ok();
    });

    match get_local_ip() {
        Some(ip) => status_sender.send(ServerStatus::Online { ip }),
        None => status_sender.send(ServerStatus::OnlineNoIp),
    };

    // And run forever...
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
        status_sender.send(ServerStatus::Err);
        return;
    }

    status_sender.send(ServerStatus::Offline);
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Not cached")))
}

impl Drop for Server {
    fn drop(&mut self) {
        self.tx.send(ServerCommand::Join).ok();
    }
}
