use std::{
    hash::{Hash, Hasher},
    net::IpAddr,
    sync::Arc,
};

use iced::{futures::stream::BoxStream, Subscription};
use iced_native::subscription::Recipe;

use tokio::sync::{broadcast, mpsc, watch};
use tokio::{runtime::Runtime, sync::oneshot};

use crate::{gui::Message, server::server::start_server, utils::await_option};

pub struct Server {
    status: watch::Receiver<ServerStatus>,
    server_message_tx: broadcast::Sender<ServerMessage>, // The tx needs to be stored since you have to have the tx to make more rx
    tx: mpsc::UnboundedSender<ServerCommand>,
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
    Error,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Status(ServerStatus),
    NewConnection { id: u32 },
    ClosedConnection { id: u32 },
}

impl Server {
    /// Create a new server
    pub fn new(runtime: Arc<Runtime>) -> Server {
        let (tx, server_rx) = mpsc::unbounded_channel();
        let (server_tx, rx) = watch::channel(ServerStatus::Offline);
        let (server_message_tx, _) = broadcast::channel(32);

        runtime.spawn(run_server(
            Arc::clone(&runtime),
            server_rx,
            server_tx,
            server_message_tx.clone(),
        ));

        Server {
            status: rx,
            tx,
            server_message_tx,
        }
    }

    /// Create a subscription to the server's status
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::from_recipe(ServerSubscription {
            id: rand::random(),
            status: self.status.clone(),
            server_messages: self.server_message_tx.subscribe(),
        })
    }

    /// Send the server a command
    pub fn send(&self, command: ServerCommand) {
        self.tx.send(command).ok();
    }
}

/// A subscription to the server's status
struct ServerSubscription {
    id: u32,
    status: watch::Receiver<ServerStatus>,
    server_messages: broadcast::Receiver<ServerMessage>,
}

/// A Recipe is iced's way of letting you tell the GUI about changes to background tasks
impl<H: Hasher, I> Recipe<H, I> for ServerSubscription {
    type Output = Message;

    /// A unique identifier for this recipe instance
    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.id.hash(state);
    }

    /// Stream messages to the GUI, iced will await futures in the stream and send the message to the GUI when they resolve.
    ///
    /// In this case, it just awaits server status changes
    fn stream(self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(iced_futures::futures::stream::unfold(
            self,
            move |mut server| async move {
                let message = tokio::select! {
                    Ok(_) = server.status.changed() => {
                        let status = *server.status.borrow();

                        Message::ServerStatus(status)
                    }

                    Ok(msg) = server.server_messages.recv() => {
                        Message::ServerMessage(msg)
                    }

                    else => Message::DoNothing
                };

                Some((message, server))
            },
        ))
    }
}

/// This function responds to server commands, and sends the server's status
///
/// rx receives commands (it has to be mut since receiving data requires an &mut reference), tx transmits the server's status
async fn run_server(
    runtime: Arc<Runtime>,
    mut rx: mpsc::UnboundedReceiver<ServerCommand>,
    status_sender: watch::Sender<ServerStatus>,
    server_message_sender: broadcast::Sender<ServerMessage>,
) {
    println!("Starting server thread");

    status_sender.send(ServerStatus::Offline).ok();

    let mut port: u16 = 8000;

    // A oneshot that when a value is transmitted, stops the server
    let mut server_canceller: Option<oneshot::Sender<()>> = None;

    let mut signal_receiver: Option<mpsc::UnboundedReceiver<ServerMessage>> = None;

    loop {
        // Wait for either receiving a command, or receiving an update to the server's status
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
                        status_sender.send(ServerStatus::Restarting).ok();

                        if let Some(canceller) = server_canceller {
                            canceller.send(()).ok();
                        }

                        let (canceller, cancel_signal) = oneshot::channel();

                        server_canceller = Some(canceller);

                        let (status_tx, status_rx) = mpsc::unbounded_channel();

                        signal_receiver = Some(status_rx);

                        runtime.spawn(start_server(port, cancel_signal, status_tx, Arc::clone(&runtime)));
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

            Some(maybe_signal) = await_option(signal_receiver.as_mut().map(|v| v.recv())) => {
                let signal = match maybe_signal {
                    Some(v) => v,
                    None => break,
                };

                match signal {
                    ServerMessage::Status(status) => {
                        status_sender.send(status).ok();
                    },
                    v => {server_message_sender.send(v).ok();},
                }
            }
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.tx.send(ServerCommand::Join).ok();
    }
}
