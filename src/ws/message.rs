use std::fmt::Display;

pub mod login;
pub mod ping;
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
    Orders,
    Fills,
    Markets,
    #[serde(rename = "orderbookGrouped")]
    GroupedOrderbook,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Orderbook => write!(f, "orderbook"),
            Channel::Trades => write!(f, "trades"),
            Channel::Ticker => write!(f, "ticker"),
            Channel::Orders => write!(f, "orders"),
            Channel::Fills => write!(f, "fills"),
            Channel::Markets => write!(f, "markets"),
            Channel::GroupedOrderbook => write!(f, "grouped orderbook"),
        }
    }
}

pub trait WebSocketApiMessage {}
