use crate::rest::model::side::Side;
use crate::ws::incoming_message::UpdateType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct TradesUpdateMessage {
    pub market: String,
    pub trades: Vec<TradeItem>,
}

#[derive(Debug)]
pub struct TradeItem {
    pub id: u64,
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
    data: Vec<RawTradesUpdateMessageDataItem>,
}

#[derive(Debug, serde::Deserialize)]
pub enum TradesChannel {
    #[serde(rename = "trades")]
    Trades,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTradesUpdateMessageDataItem {
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
            market: raw_trades_update_message.market,
            trades: raw_trades_update_message
                .data
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<RawTradesUpdateMessageDataItem> for TradeItem {
    fn from(raw_trades_update_message: RawTradesUpdateMessageDataItem) -> Self {
        Self {
            id: raw_trades_update_message.id,
            price: raw_trades_update_message.price,
            size: raw_trades_update_message.size,
            side: raw_trades_update_message.side,
            liquidation: raw_trades_update_message.liquidation,
            time: raw_trades_update_message.time,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/ws/trades_update_message.json");
        let _trades_update_message: super::RawTradesUpdateMessage =
            serde_json::from_str(json).unwrap();
    }
}
