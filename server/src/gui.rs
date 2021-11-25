use std::sync::Arc;

use iced_native::widget::*;
use iced::{Application, Clipboard, Color, Command, Length, Subscription, executor};
use tokio::runtime::Runtime;

use crate::server::{Server, ServerStatus};

pub struct Gui {
    server_status: ServerStatus,
    server: Server,
    runtime: Arc<Runtime>
}

#[derive(Debug)]
pub enum Message {
    None,
    ServerStatus(ServerStatus),
}

impl Application for Gui {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: ()) -> (Gui, Command<Message>) {
        let runtime = Arc::new(tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap());

        (Gui {
            server_status: ServerStatus::Offline,
            server: Server::new(Arc::clone(&runtime)),
            runtime,
        }, Command::none())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.server.subscription()
    }

    fn title(&self) -> String {
        "DnD stuff".to_string()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ServerStatus(status) => self.server_status = status,
            Message::None => {}
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Container::new(
            Text::new(
                match &self.server_status {
                    ServerStatus::Offline => "Server is offline".to_owned(),
                    ServerStatus::Restarting => "Server is restarting".to_owned(),
                    ServerStatus::OnlineNoIp => "Couldn't get local IP address".to_owned(),
                    ServerStatus::Online { ip } => format!("Server is running, your local IP address is {}", ip),
                    ServerStatus::Err => "The server threw an error".to_owned(),
                }
            ).color(Color::WHITE)
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }
}