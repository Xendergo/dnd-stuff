use std::thread;

use crate::gui::create_window;
#[macro_use] extern crate rocket;

mod gui;
mod utils;

#[launch]
fn rocket() -> _ {
    thread::spawn(|| {
        create_window();
    });
    
    rocket::build().mount("/", routes! [ test ])
}

#[get("/")]
fn test() -> &'static str {
    "test"
}