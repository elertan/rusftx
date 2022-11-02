pub mod fills;
pub mod markets;
pub mod orders;
pub mod pong;
pub mod subscribed;
pub mod ticker;
pub mod trades;
pub mod unsubscribed;

use crate::ws::incoming_message::fills::{FillsUpdatedMessage, RawFillsUpdatedMessage};
use crate::ws::incoming_message::markets::{MarketsMessage, RawMarketsMessage};
use crate::ws::incoming_message::orders::{OrdersUpdateMessage, RawOrdersUpdateMessage};
use crate::ws::incoming_message::subscribed::{RawSubscribedMessage, SubscribedMessage};
use crate::ws::incoming_message::ticker::{RawTickerUpdateMessage, TickerUpdateMessage};
use crate::ws::incoming_message::trades::{RawTradesUpdateMessage, TradesUpdateMessage};
use crate::ws::incoming_message::unsubscribed::{RawUnsubscribedMessage, UnsubscribedMessage};
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
    Unsubscribed(UnsubscribedMessage),
    TradesUpdate(TradesUpdateMessage),
    TickerUpdate(TickerUpdateMessage),
    FillsUpdate(FillsUpdatedMessage),
    OrdersUpdate(OrdersUpdateMessage),
    Markets(MarketsMessage),
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum RawIncomingWebSocketApiMessage {
    Pong(RawIncomingWebSocketApiPongMessage),
    Subscribed(RawSubscribedMessage),
    Unsubscribed(RawUnsubscribedMessage),
    TradesUpdate(RawTradesUpdateMessage),
    TickerUpdate(RawTickerUpdateMessage),
    FillsUpdate(RawFillsUpdatedMessage),
    OrdersUpdate(RawOrdersUpdateMessage),
    Markets(RawMarketsMessage),
}

impl From<RawIncomingWebSocketApiMessage> for IncomingWebSocketApiMessage {
    fn from(raw_incoming_web_socket_api_message: RawIncomingWebSocketApiMessage) -> Self {
        match raw_incoming_web_socket_api_message {
            RawIncomingWebSocketApiMessage::Pong(_) => IncomingWebSocketApiMessage::Pong,
            RawIncomingWebSocketApiMessage::Subscribed(raw_subscribed_message) => {
                IncomingWebSocketApiMessage::Subscribed(raw_subscribed_message.into())
            }
            RawIncomingWebSocketApiMessage::Unsubscribed(raw_unsubscribed_message) => {
                IncomingWebSocketApiMessage::Unsubscribed(raw_unsubscribed_message.into())
            }
            RawIncomingWebSocketApiMessage::TradesUpdate(raw_trades_update_message) => {
                IncomingWebSocketApiMessage::TradesUpdate(raw_trades_update_message.into())
            }
            RawIncomingWebSocketApiMessage::TickerUpdate(raw_ticker_update_message) => {
                IncomingWebSocketApiMessage::TickerUpdate(raw_ticker_update_message.into())
            }
            RawIncomingWebSocketApiMessage::FillsUpdate(raw_fills_update_message) => {
                IncomingWebSocketApiMessage::FillsUpdate(raw_fills_update_message.into())
            }
            RawIncomingWebSocketApiMessage::OrdersUpdate(raw_orders_update_message) => {
                IncomingWebSocketApiMessage::OrdersUpdate(raw_orders_update_message.into())
            }
            RawIncomingWebSocketApiMessage::Markets(raw_markets_message) => {
                IncomingWebSocketApiMessage::Markets(raw_markets_message.into())
            }
        }
    }
}
