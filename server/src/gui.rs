use std::sync::Arc;

use iced::{executor, Align, Application, Clipboard, Color, Column, Command, Length, Subscription};
use iced_native::widget::*;
use tokio::runtime::Runtime;

use crate::server::{Server, ServerCommand, ServerStatus};

use self::InputChanged::*;
use crate::server::ServerStatus::*;
use crate::styling::{self, PADDING};
use Message::*;
use ServerCommand::*;

pub struct Gui {
    server_status: ServerStatus,
    server: Server,
    runtime: Arc<Runtime>,
    widgets: Widgets,
}

#[derive(Default)]
struct Widgets {
    restart_server: button::State,
    stop_server: button::State,
    port: text_input::State,
    port_number: String,
}

#[derive(Debug, Clone)]
pub enum InputChanged {
    PortNumber(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ServerStatus(ServerStatus),
    ServerCommand(ServerCommand),
    InputChanged(InputChanged),
}

impl Application for Gui {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Message>) {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
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

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            ServerStatus(status) => {
                self.server_status = status;
            }

            ServerCommand(command) => {
                self.server.send(command);
            }

            InputChanged(input) => match input {
                PortNumber(number) => {
                    self.server.send(SwitchPort {
                        port: number.parse().unwrap_or(8000),
                    });

                    self.widgets.port_number = number;
                }
            },

            None => {}
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut server_options: Vec<iced::Element<'_, Self::Message>> = Vec::new();

        server_options.push(
            Button::new(
                &mut self.widgets.restart_server,
                Text::new(match self.server_status {
                    Offline => "Start",
                    _ => "Restart",
                }),
            )
            .on_press(ServerCommand(Restart))
            .padding(PADDING)
            .style(styling::Button())
            .into(),
        );

        if let Online { ip: _ } | OnlineNoIp = self.server_status {
            server_options.push(
                Button::new(&mut self.widgets.stop_server, Text::new("Stop"))
                    .on_press(ServerCommand(Stop))
                    .padding(PADDING)
                    .style(styling::Button())
                    .into(),
            );
        }

        server_options.push(
            TextInput::new(
                &mut self.widgets.port,
                "Port",
                &self.widgets.port_number.to_string(),
                |port| {
                    let filtered = port
                        .trim_matches(|char: char| !char.is_ascii_digit())
                        .to_owned();

                    if filtered.parse::<u16>().is_ok() || filtered == "" {
                        InputChanged(PortNumber(filtered))
                    } else {
                        None
                    }
                },
            )
            .width(Length::Units(12 * 5))
            .padding(PADDING)
            .style(styling::TextInput())
            .into(),
        );

        Column::with_children(vec![
            Text::new(format!(
                "Server status: {}",
                match &self.server_status {
                    Offline => "Offline".to_owned(),
                    Restarting => "Restarting".to_owned(),
                    OnlineNoIp => "Online, couldn't get local IP address".to_owned(),
                    Online { ip } => format!("Online, your local IP address is {}", ip),
                    Err => "The server threw an error".to_owned(),
                }
            ))
            .color(Color::WHITE)
            .into(),
            Row::with_children(server_options).spacing(16).into(),
        ])
        .spacing(16)
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
