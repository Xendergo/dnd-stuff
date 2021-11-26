use std::{convert::Infallible, net::SocketAddr};

use hyper::{service, Body, Request, Response, Server};
use tokio::sync::{mpsc, oneshot};

use crate::{server::ServerStatus, utils::get_local_ip};

pub(super) async fn start_server(
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
    let server = Server::bind(&addr).serve(make_service);

    let graceful = server.with_graceful_shutdown(async {
        cancel_signal.await.ok();
    });

    match get_local_ip() {
        Some(ip) => status_sender.send(ServerStatus::Online { ip }),
        None => status_sender.send(ServerStatus::OnlineNoIp),
    }
    .ok();

    // And run forever...
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
        status_sender.send(ServerStatus::Err).ok();
        return;
    }

    status_sender.send(ServerStatus::Offline).ok();
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Not cached")))
}
