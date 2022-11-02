pub mod pong;
pub mod subscribed;
pub mod ticker;

use crate::ws::incoming_message::subscribed::{RawSubscribedMessage, SubscribedMessage};
use crate::ws::incoming_message::ticker::{RawTickerUpdateMessage, TickerUpdateMessage};
use pong::RawIncomingWebSocketApiPongMessage;

#[derive(Debug, serde::Deserialize)]
pub enum UpdateType {
    #[serde(rename = "update")]
    Update,
}

#[derive(Debug)]
pub enum IncomingWebSocketApiMessage {
    Pong,
    Subscribed(SubscribedMessage),
    TickerUpdate(TickerUpdateMessage),
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum RawIncomingWebSocketApiMessage {
    Pong(RawIncomingWebSocketApiPongMessage),
    Subscribed(RawSubscribedMessage),
    TickerUpdate(RawTickerUpdateMessage),
}

impl From<RawIncomingWebSocketApiMessage> for IncomingWebSocketApiMessage {
    fn from(raw_incoming_web_socket_api_message: RawIncomingWebSocketApiMessage) -> Self {
        match raw_incoming_web_socket_api_message {
            RawIncomingWebSocketApiMessage::Pong(_) => IncomingWebSocketApiMessage::Pong,
            RawIncomingWebSocketApiMessage::Subscribed(raw_subscribed_message) => {
                IncomingWebSocketApiMessage::Subscribed(raw_subscribed_message.into())
            }
            RawIncomingWebSocketApiMessage::TickerUpdate(raw_ticker_update_message) => {
                IncomingWebSocketApiMessage::TickerUpdate(raw_ticker_update_message.into())
            }
        }
    }
}
