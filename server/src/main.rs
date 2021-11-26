use gui::Gui;
use iced::{Application, Settings};

mod gui;
mod server;
mod styling;
mod utils;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
