use std::{net::SocketAddr, sync::Arc};

use hyper::{service, Body, Request, Response, Server};
use hyper_tungstenite::{
    tungstenite::{Error, Message},
    HyperWebsocket,
};
use iced::futures::{SinkExt, StreamExt};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot},
};

use crate::{server::ServerStatus, utils::get_local_ip};

pub(super) async fn start_server(
    port: u16,
    cancel_signal: oneshot::Receiver<()>,
    status_sender: mpsc::UnboundedSender<ServerStatus>,
    runtime: Arc<Runtime>,
) {
    println!("Starting server");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Enter the tokio runtime, the runtime can't get sent in a closure to the request handler because of lifetime issues
    let guard = runtime.handle().enter();

    // Register the request handler
    let make_service = service::make_service_fn(|_conn| async {
        Ok::<_, Error>(service::service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_service);

    // Configure the server to stop when the oneshot value is received
    let graceful = server.with_graceful_shutdown(async {
        cancel_signal.await.ok();
    });

    match get_local_ip() {
        Some(ip) => status_sender.send(ServerStatus::Online { ip }),
        None => status_sender.send(ServerStatus::OnlineNoIp),
    }
    .ok();

    // Run forever, until there's an error or the server shuts down
    match graceful.await {
        Ok(_) => {
            println!("Server stopped");
            status_sender.send(ServerStatus::Offline).ok();
        }

        Err(e) => {
            eprintln!("Server errored: {}", e);
            status_sender.send(ServerStatus::Err).ok();
        }
    }

    // Stay in the tokio runtime until the server is closed instead of exiting immediately
    drop(guard)
}

/// Handle requests sent to the server
async fn handle_request(request: Request<Body>) -> Result<Response<Body>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        println!("Received upgrade request");
        let (response, websocket) = hyper_tungstenite::upgrade(request, None)?;

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(websocket).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });

        return Ok(response);
    }

    Ok(Response::new(Body::from("Hello HTTP!")))
}

/// Manage a websocket connection
async fn serve_websocket(websocket: HyperWebsocket) -> Result<(), Error> {
    println!("Websocket successfully connected");
    let mut websocket = websocket.await?;
    while let Some(message) = websocket.next().await {
        match message? {
            Message::Text(msg) => {
                println!("Received text message: {}", msg);
                websocket
                    .send(Message::text("Thank you, come again."))
                    .await?;
            }
            Message::Binary(msg) => {
                println!("Received binary message: {:02X?}", msg);
                websocket
                    .send(Message::binary(b"Thank you, come again.".to_vec()))
                    .await?;
            }
            Message::Ping(msg) => {
                // No need to send a reply: tungstenite takes care of this for you.
                println!("Received ping message: {:02X?}", msg);
            }
            Message::Pong(msg) => {
                println!("Received pong message: {:02X?}", msg);
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
        }
    }

    Ok(())
}
