use std::sync::Arc;

use iced::{executor, Align, Application, Clipboard, Color, Column, Command, Length, Subscription};
use iced_native::widget::*;

use crate::server::{Server, ServerCommand, ServerMessage, ServerStatus};

use self::InputChanged::*;
use crate::server::ServerStatus::*;
use crate::styling::{self, PADDING};
use Message::*;
use ServerCommand::*;

pub struct Gui {
    server_status: ServerStatus,
    server: Server,
    widgets: Widgets,
    connections: Vec<ConnectionData>,
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
    /// Do nothing
    NoMessage,

    /// Sent by the server when its status changes
    ServerStatus(ServerStatus),

    /// Tell the server to do something
    ServerCommand(ServerCommand),

    /// Update the value of one of the inputs
    InputChanged(InputChanged),

    /// A message was received from the server
    ServerMessage(ServerMessage),
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
                widgets: Widgets::default(),
                connections: Vec::new(),
            },
            Command::none(),
        )
    }

    /// Subscribe to the server's status
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

            ServerMessage(msg) => match msg {
                ServerMessage::NewConnection { id } => {
                    let data = ConnectionData { id, shown: true };

                    match self.connections.iter().position(|v| v.id == id) {
                        Some(position) => self.connections[position] = data,
                        None => self.connections.push(data),
                    };
                }

                ServerMessage::ClosedConnection { id } => {
                    if let Some(position) = self.connections.iter().position(|v| v.id == id) {
                        self.connections[position].shown = false;
                    }
                }

                ServerMessage::Status(_) => unreachable!(), // Status messages are intercepted and sent as ServerStatus instead
            },

            NoMessage => {}
        }

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::with_children(vec![
            // Display the server status
            Text::new(format!(
                "Server status: {}",
                match &self.server_status {
                    Offline => "Offline".to_owned(),
                    Restarting => "Restarting".to_owned(),
                    OnlineNoIp => "Online, couldn't get local IP address".to_owned(),
                    Online { ip } => format!("Online, your local IP address is {}", ip),
                    Error => "The server threw an error".to_owned(),
                }
            ))
            .color(Color::WHITE)
            .into(),
            // The row of server interactions
            Row::with_children(Gui::server_interactions(
                &mut self.widgets,
                self.server_status,
            ))
            .spacing(16)
            .into(),
            Space::new(Length::Units(0), Length::Units(8)).into(),
            Text::new("Connections").color(Color::WHITE).size(30).into(),
            Column::with_children(
                self.connections
                    .iter()
                    .filter(|v| v.shown)
                    .map(|v| Row::with_children(v.view()).into())
                    .collect::<Vec<_>>(),
            )
            .into(),
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

impl Gui {
    /// The row of buttons & inputs users can interact with the server with
    fn server_interactions(
        widgets: &mut Widgets,
        server_status: ServerStatus,
    ) -> Vec<iced::Element<'_, <Self as Application>::Message>> {
        let mut server_interactions: Vec<iced::Element<'_, <Self as Application>::Message>> =
            Vec::new();

        // Start / Restart button
        server_interactions.push(
            Button::new(
                &mut widgets.restart_server,
                Text::new(match server_status {
                    Offline => "Start",
                    _ => "Restart",
                }),
            )
            .on_press(ServerCommand(Restart))
            .padding(PADDING)
            .style(styling::Button())
            .into(),
        );

        // Stop button
        if let Online { ip: _ } | OnlineNoIp = server_status {
            server_interactions.push(
                Button::new(&mut widgets.stop_server, Text::new("Stop"))
                    .on_press(ServerCommand(Stop))
                    .padding(PADDING)
                    .style(styling::Button())
                    .into(),
            );
        }

        // Port number
        server_interactions.push(
            TextInput::new(
                &mut widgets.port,
                "Port",
                &widgets.port_number.to_string(),
                |port| {
                    let filtered = port
                        .trim_matches(|char: char| !char.is_ascii_digit())
                        .to_owned();

                    if filtered.parse::<u16>().is_ok() || filtered == "" {
                        InputChanged(PortNumber(filtered))
                    } else {
                        NoMessage
                    }
                },
            )
            .width(Length::Units(12 * 5))
            .padding(PADDING)
            .style(styling::TextInput())
            .into(),
        );

        server_interactions
    }
}

struct ConnectionData {
    id: u32,
    shown: bool,
}

impl ConnectionData {
    fn view(&self) -> Vec<iced::Element<'_, <Gui as Application>::Message>> {
        let mut data: Vec<iced::Element<'_, <Gui as Application>::Message>> = Vec::new();

        data.push(Text::new(self.id.to_string()).color(Color::WHITE).into());

        data
    }
}
