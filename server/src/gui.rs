use std::sync::Arc;

use iced::{executor, Align, Application, Clipboard, Color, Column, Command, Length, Subscription};
use iced_native::widget::*;
use tokio::runtime::Runtime;

use crate::server::{Server, ServerCommand, ServerStatus};

pub struct Gui {
    server_status: ServerStatus,
    server: Server,
    runtime: Arc<Runtime>,
    widgets: Widgets,
}

#[derive(Default)]
struct Widgets {
    restart_server: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ServerStatus(ServerStatus),
    ServerCommand(ServerCommand),
}

impl Application for Gui {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: ()) -> (Gui, Command<Message>) {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        );

        (
            Gui {
                server_status: ServerStatus::Offline,
                server: Server::new(Arc::clone(&runtime)),
                runtime,
                widgets: Widgets::default(),
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.server.subscription()
    }

    fn title(&self) -> String {
        "DnD stuff".to_string()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ServerStatus(status) => {
                self.server_status = status;
            }
            Message::ServerCommand(command) => {
                self.server.send(command);
            }
            Message::None => {}
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut server_options: Vec<iced::Element<'_, Self::Message>> = Vec::new();

        server_options.push(
            Button::new(
                &mut self.widgets.restart_server,
                Text::new(match self.server_status {
                    ServerStatus::Offline => "Start",
                    _ => "Restart",
                }),
            )
            .on_press(Message::ServerCommand(ServerCommand::Restart))
            .into(),
        );

        Column::with_children(vec![
            Text::new(format!(
                "Server status: {}",
                match &self.server_status {
                    ServerStatus::Offline => "Offline".to_owned(),
                    ServerStatus::Restarting => "Restarting".to_owned(),
                    ServerStatus::OnlineNoIp => "Online, couldn't get local IP address".to_owned(),
                    ServerStatus::Online { ip } =>
                        format!("Online, your local IP address is {}", ip),
                    ServerStatus::Err => "The server threw an error".to_owned(),
                }
            ))
            .color(Color::WHITE)
            .into(),
            Row::with_children(server_options).into(),
        ])
        .align_items(Align::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(16)
        .into()
    }

    fn background_color(&self) -> Color {
        Color::BLACK
    }
}
