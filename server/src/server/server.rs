use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{service, upgrade::Upgraded, Body, Request, Response, Server};
use hyper_tungstenite::{
    tungstenite::{Error, Message},
    HyperWebsocket, WebSocketStream,
};
use iced::futures::{SinkExt, StreamExt};
use tokio::{
    runtime::Runtime,
    sync::{
        broadcast,
        mpsc::{self, UnboundedSender},
        oneshot,
    },
};

use serde::{Deserialize, Serialize};

use crate::utils::get_local_ip;

use super::{
    ServerMessage::{self, *},
    ServerStatus::*,
};

#[derive(Debug, Clone)]
enum InternalMessage {
    CharacterUpdated {
        character_data: String,
        player_id: u32,
    },
}

pub(super) async fn start_server(
    port: u16,
    cancel_signal: oneshot::Receiver<()>,
    signal_sender: mpsc::UnboundedSender<ServerMessage>,
    runtime: Arc<Runtime>,
) {
    println!("Starting server");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let cloned_signal_sender = signal_sender.clone();

    let (internal_message_broadcaster, _) = broadcast::channel(32);

    // Register the request handler
    let make_service = service::make_service_fn(move |_conn| {
        let runtime = Arc::clone(&runtime);
        let signal_sender = signal_sender.clone();
        let internal_message_broadcaster = internal_message_broadcaster.clone();

        let service = service::service_fn(move |req| {
            handle_request(
                req,
                Arc::clone(&runtime),
                signal_sender.clone(),
                internal_message_broadcaster.clone(),
            )
        });

        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_service);

    // Configure the server to stop when the oneshot value is received
    let graceful = server.with_graceful_shutdown(async {
        cancel_signal.await.ok();
    });

    match get_local_ip() {
        Some(ip) => cloned_signal_sender.send(Status(Online { ip })),
        None => cloned_signal_sender.send(Status(OnlineNoIp)),
    }
    .ok();

    // Run forever, until there's an error or the server shuts down
    match graceful.await {
        Ok(_) => {
            println!("Server stopped");
            cloned_signal_sender.send(Status(Offline)).ok();
        }

        Err(e) => {
            eprintln!("Server errored: {}", e);
            cloned_signal_sender.send(Status(Error)).ok();
        }
    }
}

/// Handle requests sent to the server
async fn handle_request(
    request: Request<Body>,
    runtime: Arc<Runtime>,
    signal_sender: UnboundedSender<ServerMessage>,
    internal_message_broadcaster: broadcast::Sender<InternalMessage>,
) -> Result<Response<Body>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        println!("Received upgrade request");
        let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;

        // Spawn a task to handle the websocket connection.
        runtime.spawn(async move {
            if let Err(e) = serve_websocket(
                websocket,
                signal_sender.clone(),
                internal_message_broadcaster.clone(),
            )
            .await
            {
                eprintln!("Error in websocket connection: {}", e);
            }
        });

        return Ok(response);
    }

    Ok(Response::new(Body::from("Hello HTTP!")))
}

#[derive(Debug, Deserialize)]
enum FromClientMessage {
    Id(Option<u32>),
    CharacterUpdated { data: String },
}

#[derive(Debug, Serialize)]
enum ToClientMessage {
    Id(u32),
    CharacterUpdated { data: String, player_id: u32 },
}

/// Manage a websocket connection
async fn serve_websocket(
    websocket: HyperWebsocket,
    signal_sender: UnboundedSender<ServerMessage>,
    internal_message_broadcaster: broadcast::Sender<InternalMessage>,
) -> Result<(), Error> {
    println!("Websocket successfully connected");

    let mut internal_message_receiver = internal_message_broadcaster.subscribe();

    let mut websocket = websocket.await?;

    let mut id: Option<u32> = None;

    loop {
        tokio::select! {
            maybe_message = websocket.next() => {
                // Exit the loop if websocket.next returns none, because if it does, the websocket was closed

                let message = if let Some(v) = maybe_message {v} else {break};

                match message? {
                    Message::Text(msg_raw) => {
                        println!("{:?}", msg_raw);
                        let msg: FromClientMessage = match serde_json::from_str(&msg_raw) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };
                        println!("{:?}", msg);

                        match msg {
                            FromClientMessage::Id(v) => {
                                if id.is_some() {
                                    continue;
                                }

                                match v {
                                    Some(v) => id = Some(v),
                                    None => {
                                        let id_value = rand::random();

                                        send(&mut websocket, ToClientMessage::Id(id_value)).await?;

                                        id = Some(id_value);
                                    }
                                }

                                signal_sender.send(NewConnection { id: id.unwrap() }).ok();
                            }

                            FromClientMessage::CharacterUpdated { data } => {
                                internal_message_broadcaster
                                    .send(InternalMessage::CharacterUpdated {
                                        character_data: data,
                                        player_id: match id {
                                            Some(v) => v,
                                            None => continue,
                                        },
                                    })
                                    .ok();
                            }
                        }
                    }

                    Message::Close(msg) => {
                        // No need to send a reply: tungstenite takes care of this for you.
                        if let Some(msg) = &msg {
                            println!(
                                "Received close message with code {} and message: {}",
                                msg.code, msg.reason
                            );
                        } else {
                            println!("Received close message");
                        }
                    }

                    _ => {}
                }
            }

            maybe_internal_message = internal_message_receiver.recv() => {
                let internal_message = match maybe_internal_message {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                match internal_message {
                    InternalMessage::CharacterUpdated { character_data, player_id } => {
                        send(&mut websocket, ToClientMessage::CharacterUpdated { data: character_data, player_id }).await?;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn send(
    websocket: &mut WebSocketStream<Upgraded>,
    message: ToClientMessage,
) -> Result<(), Error> {
    websocket
        .send(Message::Text(serde_json::to_string(&message).unwrap()))
        .await
}

// #[cfg(test)]
// mod tests {
//     use super::ClientMessage::*;

//     #[test]
//     fn bruh() {
//         println!("{:?}", serde_json::to_string(&Id(None)));
//     }
// }
