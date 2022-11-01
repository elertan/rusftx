use crate::ws::message::login::LoginMessage;
use crate::ws::message::subscribe::SubscribeMessage;
use crate::ws::message::unsubscribe::UnsubscribeMessage;

pub mod login;
pub mod subscribe;
pub mod unsubscribe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Ping,
    Login,
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Orderbook,
    Trades,
    Ticker,
}

pub enum Message {
    Ping,
    Login(LoginMessage),
    Subscribe(SubscribeMessage),
    Unsubscribe(UnsubscribeMessage),
}

impl From<&Message> for Operation {
    fn from(message: &Message) -> Self {
        match message {
            Message::Ping => Operation::Ping,
            Message::Login(_) => Operation::Login,
            Message::Subscribe(_) => Operation::Subscribe,
            Message::Unsubscribe(_) => Operation::Unsubscribe,
        }
    }
}
