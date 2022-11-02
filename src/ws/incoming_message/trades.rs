use crate::rest::model::side::Side;
use crate::ws::incoming_message::UpdateType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct TradesUpdateMessage {
    pub id: u64,
    pub market: String,
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub liquidation: bool,
    pub time: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTradesUpdateMessage {
    #[serde(rename = "type")]
    _type: UpdateType,
    #[serde(rename = "channel")]
    _channel: TradesChannel,
    market: String,
    data: RawTradesUpdateMessageData,
}

#[derive(Debug, serde::Deserialize)]
pub enum TradesChannel {
    #[serde(rename = "trades")]
    Trades,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTradesUpdateMessageData {
    id: u64,
    price: Decimal,
    size: Decimal,
    side: Side,
    liquidation: bool,
    time: DateTime<Utc>,
}

impl From<RawTradesUpdateMessage> for TradesUpdateMessage {
    fn from(raw_trades_update_message: RawTradesUpdateMessage) -> Self {
        Self {
            id: raw_trades_update_message.data.id,
            market: raw_trades_update_message.market,
            price: raw_trades_update_message.data.price,
            size: raw_trades_update_message.data.size,
            side: raw_trades_update_message.data.side,
            liquidation: raw_trades_update_message.data.liquidation,
            time: raw_trades_update_message.data.time,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/ws/trades_update_message.json");
        let trades_update_message: super::RawTradesUpdateMessage =
            serde_json::from_str(json).unwrap();
    }
}
