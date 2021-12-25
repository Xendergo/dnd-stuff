use std::{collections::HashMap, sync::Arc};

use hyper::upgrade::Upgraded;
use hyper_tungstenite::{
    tungstenite::{Error, Message},
    WebSocketStream,
};
use iced::futures::SinkExt;
use tokio::sync::{broadcast, mpsc::UnboundedSender, RwLock};

use crate::server::server::FromClientMessage;

use super::{
    server::{InternalMessage, ToClientMessage},
    ServerMessage::{self, *},
};

async fn send(
    websocket: &mut WebSocketStream<Upgraded>,
    message: ToClientMessage,
) -> Result<(), Error> {
    websocket
        .send(Message::Text(serde_json::to_string(&message).unwrap()))
        .await
}

pub(super) async fn on_message(
    message: Result<Message, Error>,
    websocket: &mut WebSocketStream<Upgraded>,
    id: &mut Option<u32>,
    signal_sender: &UnboundedSender<ServerMessage>,
    internal_message_broadcaster: &broadcast::Sender<InternalMessage>,
    character_states: &Arc<RwLock<HashMap<String, (u32, String)>>>,
) -> Result<(), Error> {
    match message? {
        Message::Text(msg_raw) => {
            // println!("{}", msg_raw);
            let msg: FromClientMessage = match serde_json::from_str(&msg_raw) {
                Ok(v) => v,
                Err(_) => return Ok(()),
            };

            match msg {
                FromClientMessage::RequestId {} => {
                    requested_id(websocket, character_states, id, signal_sender).await?
                }

                FromClientMessage::Id { id: new_id } => {
                    received_id(websocket, character_states, new_id, id, signal_sender).await?
                }

                FromClientMessage::CharacterUpdated { data } => {
                    character_updated(data, internal_message_broadcaster, *id, character_states)
                        .await
                }
            }
        }

        Message::Close(_) => {
            let id = match id {
                Some(v) => v,
                None => return Ok(()),
            };

            let mut characters = character_states.write().await;

            let mut to_remove = Vec::new();

            for (key, (character_id, _)) in characters.iter() {
                if character_id == id {
                    to_remove.push(key.clone());
                }
            }

            for key in to_remove {
                characters.remove(&key);
            }
        }

        _ => {}
    }

    Ok(())
}

async fn requested_id(
    websocket: &mut WebSocketStream<Upgraded>,
    character_states: &Arc<RwLock<HashMap<String, (u32, String)>>>,
    id: &mut Option<u32>,
    signal_sender: &UnboundedSender<ServerMessage>,
) -> Result<(), Error> {
    let id_value = rand::random();

    id_assigned(websocket, character_states, id_value, id, signal_sender).await?;

    send(websocket, ToClientMessage::Id { id: id_value }).await?;

    Ok(())
}

async fn received_id(
    websocket: &mut WebSocketStream<Upgraded>,
    character_states: &Arc<RwLock<HashMap<String, (u32, String)>>>,
    new_id: u32,
    id: &mut Option<u32>,
    signal_sender: &UnboundedSender<ServerMessage>,
) -> Result<(), Error> {
    if id.is_some() {
        return Ok(());
    }

    id_assigned(websocket, character_states, new_id, id, signal_sender).await?;

    Ok(())
}

async fn id_assigned(
    websocket: &mut WebSocketStream<Upgraded>,
    character_states: &Arc<RwLock<HashMap<String, (u32, String)>>>,
    new_id: u32,
    id: &mut Option<u32>,
    signal_sender: &UnboundedSender<ServerMessage>,
) -> Result<(), Error> {
    *id = Some(new_id);

    signal_sender.send(NewConnection { id: id.unwrap() }).ok();

    let characters = character_states.read().await;

    for (player_id, data) in characters.values() {
        send(
            websocket,
            ToClientMessage::CharacterUpdated {
                data: data.to_string(),
                player_id: *player_id,
            },
        )
        .await?;
    }

    Ok(())
}

async fn character_updated(
    data: String,
    internal_message_broadcaster: &broadcast::Sender<InternalMessage>,
    id: Option<u32>,
    character_states: &Arc<RwLock<HashMap<String, (u32, String)>>>,
) {
    let name = match serde_json::from_str(&data) {
        Ok(serde_json::Value::Object(obj)) => {
            let maybe_name = obj.get("name");

            match maybe_name {
                Some(serde_json::Value::String(name)) => name.clone(),
                _ => return,
            }
        }

        _ => return,
    };

    println!("id: {:?}", &id);

    internal_message_broadcaster
        .send(InternalMessage::CharacterUpdated {
            character_data: data.clone(),
            player_id: match id {
                Some(v) => v,
                None => return,
            },
        })
        .ok();

    let mut states = character_states.write().await;
    states.insert(name, (id.unwrap(), data));
    drop(states);
}

pub(super) async fn received_internal_message(
    msg: InternalMessage,
    websocket: &mut WebSocketStream<Upgraded>,
) -> Result<(), Error> {
    match msg {
        InternalMessage::CharacterUpdated {
            character_data,
            player_id,
        } => {
            send(
                websocket,
                ToClientMessage::CharacterUpdated {
                    data: character_data,
                    player_id,
                },
            )
            .await?;
        }
    }

    Ok(())
}
