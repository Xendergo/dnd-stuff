use gui::Gui;
use iced::{Application, Settings};

extern crate tiny_http;

mod gui;
mod utils;
mod server;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}