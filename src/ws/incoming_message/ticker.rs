use crate::ws::incoming_message::UpdateType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_with::{serde_as, TimestampSecondsWithFrac};

#[derive(Debug)]
pub struct TickerUpdateMessage {
    pub market: String,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub bid_size: Option<Decimal>,
    pub ask_size: Option<Decimal>,
    pub last: Option<Decimal>,
    pub time: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTickerUpdateMessage {
    #[serde(rename = "channel")]
    pub _channel: TickerChannel,
    #[serde(rename = "type")]
    pub _type: UpdateType,
    pub market: String,
    pub data: RawTickerUpdateMessageData,
}

#[derive(Debug, serde::Deserialize)]
pub enum TickerChannel {
    #[serde(rename = "ticker")]
    Ticker,
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTickerUpdateMessageData {
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub bid_size: Option<Decimal>,
    pub ask_size: Option<Decimal>,
    pub last: Option<Decimal>,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub time: DateTime<Utc>,
}

impl From<RawTickerUpdateMessage> for TickerUpdateMessage {
    fn from(raw_ticker_update_message: RawTickerUpdateMessage) -> Self {
        Self {
            market: raw_ticker_update_message.market,
            bid: raw_ticker_update_message.data.bid,
            ask: raw_ticker_update_message.data.ask,
            bid_size: raw_ticker_update_message.data.bid_size,
            ask_size: raw_ticker_update_message.data.ask_size,
            last: raw_ticker_update_message.data.last,
            time: raw_ticker_update_message.data.time,
        }
    }
}

mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/ws/ticker_update_message.json");
        let _ticker_update_message: super::RawTickerUpdateMessage =
            serde_json::from_str(json).unwrap();
    }
}
