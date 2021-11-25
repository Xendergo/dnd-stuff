use std::{hash::{Hasher, Hash}, net::IpAddr, sync::{Arc, atomic::{AtomicU32, Ordering}}, thread, time::Duration};

use iced::{Subscription, futures::stream::BoxStream};
use iced_native::subscription::Recipe;
use tiny_http::{Response, Server as HttpServer};

use crossbeam::channel::{self, Sender, Receiver};
use tokio::runtime::Runtime;

use crate::{gui::Message, utils::get_local_ip};

static ID: AtomicU32 = AtomicU32::new(0);

pub struct Server {
    rx: Receiver<ServerStatus>,
    tx: Sender<ServerCommand>,
    runtime: Arc<Runtime>
}

#[derive(Debug, Clone, Copy)]
pub enum ServerCommand {
    SwitchPort { port: u32 },
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
        let (tx, server_rx) = channel::unbounded();
        let (server_tx, rx) = channel::unbounded();

        thread::spawn(move || {
            run_server(server_rx, server_tx)
        });
        
        Server {
            rx,
            tx,
            runtime,
        }
    }
    
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::from_recipe(ServerSubscription {
            id: ID.fetch_add(1, Ordering::Relaxed),
            rx: self.rx.clone(),
            runtime: Arc::clone(&self.runtime),
        })
    }

    pub fn send(&self, command: ServerCommand) {
        self.tx.send(command).ok();
    }
}

struct ServerSubscription {
    id: u32,
    rx: Receiver<ServerStatus>,
    runtime: Arc<Runtime>
}

impl<H: Hasher, I> Recipe<H, I> for ServerSubscription {
    type Output = Message;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.id.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<I>) -> BoxStream<Self::Output> {
        Box::pin(iced_futures::futures::stream::unfold(self, move |server| async move {
            Some(Arc::clone(&server.runtime).spawn_blocking(move || {
                let maybe_msg = server.rx.recv().ok();
        
                (match maybe_msg {
                    Some(msg) => Message::ServerStatus(msg),
                    None => Message::None,
                }, server)
            }).await.unwrap())
        }))
    }
}

fn run_server(rx: Receiver<ServerCommand>, tx: Sender<ServerStatus>) {
    println!("Start server");
    tx.send(ServerStatus::Offline).ok();
    
    let mut port: u32 = 8000;
    
    let mut server: Option<HttpServer> = None;
    
    loop {
        if let Some(server) = &server {
            // Idk what kind of errors these can make, hopefully ignoring the errors won't cause problems
    
            let maybe_req = server.recv_timeout(Duration::from_millis(500));
    
            if let Ok(Some(req)) = maybe_req {
                let response = Response::from_string("bruh");
    
                let _ = req.respond(response);
            }
        }
    
        let commands = rx.try_iter();
    
        for command in commands {
            match command {
                ServerCommand::SwitchPort { port: new_port } => {
                    port = new_port;
                },
                ServerCommand::Restart => {
                    tx.send(ServerStatus::Restarting).ok();

                    server = None;
    
                    let maybe_server = HttpServer::http(format!("0.0.0.0:{}", port));
    
                    match maybe_server {
                        Ok(definitely_server) => {
                            server = Some(definitely_server);
    
                            tx.send(match get_local_ip() {
                                Some(ip) => {
                                    ServerStatus::Online { ip }
                                },
                                None => {
                                    ServerStatus::OnlineNoIp
                                }
                            }).ok();
                        },
                        Err(e) => {
                            tx.send(ServerStatus::Err).ok();
    
                            println!("{}", e);
                        }
                    }
                },
                ServerCommand::Stop => {
                    tx.send(ServerStatus::Offline).ok();

                    server = None;
                },
                ServerCommand::Join => {
                    break
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