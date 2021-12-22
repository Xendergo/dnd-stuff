use std::{collections::HashMap, convert::Infallible, net::SocketAddr, sync::Arc};

use hyper::{service, Body, Request, Response, Server};
use hyper_tungstenite::{tungstenite::Error, HyperWebsocket};
use iced::futures::StreamExt;
use tokio::{
    runtime::Runtime,
    sync::{
        broadcast,
        mpsc::{self, UnboundedSender},
        oneshot, RwLock,
    },
};

use serde::{Deserialize, Serialize};

use crate::{
    server::websocket::{on_message, received_internal_message},
    utils::get_local_ip,
};

use super::{
    ServerMessage::{self, *},
    ServerStatus::*,
};

#[derive(Debug, Clone)]
pub(super) enum InternalMessage {
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

    let character_states = Arc::new(RwLock::new(HashMap::new()));

    // Register the request handler
    let make_service = service::make_service_fn(move |_conn| {
        let runtime = Arc::clone(&runtime);
        let signal_sender = signal_sender.clone();
        let internal_message_broadcaster = internal_message_broadcaster.clone();
        let character_states = Arc::clone(&character_states);

        let service = service::service_fn(move |req| {
            handle_request(
                req,
                Arc::clone(&runtime),
                signal_sender.clone(),
                internal_message_broadcaster.clone(),
                Arc::clone(&character_states),
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
    character_states: Arc<RwLock<HashMap<String, String>>>,
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
                Arc::clone(&character_states),
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
pub(super) enum FromClientMessage {
    RequestId {},
    Id { id: u32 },
    CharacterUpdated { data: String },
}

#[derive(Debug, Serialize)]
pub(super) enum ToClientMessage {
    Id { id: u32 },
    CharacterUpdated { data: String, player_id: u32 },
}

/// Manage a websocket connection
async fn serve_websocket(
    websocket: HyperWebsocket,
    signal_sender: UnboundedSender<ServerMessage>,
    internal_message_broadcaster: broadcast::Sender<InternalMessage>,
    character_states: Arc<RwLock<HashMap<String, String>>>,
) -> Result<(), Error> {
    let mut internal_message_receiver = internal_message_broadcaster.subscribe();

    let mut websocket = websocket.await?;

    let mut id: Option<u32> = None;

    loop {
        tokio::select! {
            maybe_message = websocket.next() => {
                // Exit the loop if websocket.next returns none, because if it does, the websocket was closed
                let message = if let Some(v) = maybe_message { v } else { break };

                on_message(message, &mut websocket, &mut id, &signal_sender, &internal_message_broadcaster, &character_states).await?;
            }

            maybe_internal_message = internal_message_receiver.recv() => {
                println!("Character updated transmitted internally: {:?}", maybe_internal_message);
                let internal_message = match maybe_internal_message {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                received_internal_message(internal_message, &mut websocket).await?;
            }
        }
    }

    if let Some(id) = id {
        signal_sender.send(ClosedConnection { id }).ok();
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::FromClientMessage::*;

//     #[test]
//     fn bruh() {
//         println!("{:?}", serde_json::to_string(&Id { id: 0 }));
//     }
// }
