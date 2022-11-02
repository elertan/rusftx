use crate::rest::model::market::Market;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MarketsMessage {
    pub markets: HashMap<String, Market>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawMarketsMessage {
    #[serde(rename = "channel")]
    _channel: MarketsChannel,
    #[serde(rename = "type")]
    _type: PartialType,
    data: RawMarketsMessageData,
}

#[derive(Debug, serde::Deserialize)]
pub enum MarketsChannel {
    #[serde(rename = "markets")]
    Markets,
}

#[derive(Debug, serde::Deserialize)]
pub enum PartialType {
    #[serde(rename = "partial")]
    Partial,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawMarketsMessageData {
    data: HashMap<String, Market>,
}

impl From<RawMarketsMessage> for MarketsMessage {
    fn from(raw_markets_message: RawMarketsMessage) -> Self {
        MarketsMessage {
            markets: raw_markets_message.data.data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/ws/markets_message.json");
        let _markets_message: super::RawMarketsMessage = serde_json::from_str(json).unwrap();
    }
}
