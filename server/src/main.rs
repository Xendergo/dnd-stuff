use std::{thread};

use gui::Gui;
use iced::{Sandbox, Settings};
use tiny_http::{Response, Server};

extern crate tiny_http;

mod gui;
mod utils;

fn main() -> iced::Result {
    thread::spawn(|| {
        let server = Server::http("0.0.0.0:8000").unwrap();

        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                request.method(),
                request.url(),
                request.headers()
            );
        
            let response = Response::from_string("hello world");
            request.respond(response).unwrap();
        }
    });
    
    Gui::run(Settings::default())
}