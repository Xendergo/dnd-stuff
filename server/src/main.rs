use gui::Gui;
use iced::{Application, Settings};

mod gui;
mod server;
mod utils;

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
