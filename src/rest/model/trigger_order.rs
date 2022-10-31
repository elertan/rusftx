use crate::rest::model::order::OrderType;
use crate::rest::model::side::Side;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrder {
    pub id: u64,
    pub market: String,
    pub r#type: TriggerOrderType,
    pub side: Side,
    pub trigger_price: Option<Decimal>,
    pub order_price: Option<Decimal>,
    pub size: Decimal,
    pub status: TriggerOrderStatus,
    pub reduce_only: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub future: Option<String>,
    #[deprecated]
    pub error: Option<String>,
    pub triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub order_type: Option<OrderType>,
    pub retry_until_filled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderType {
    Stop,
    TrailingStop,
    TakeProfit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerOrderStatus {
    Open,
    Cancelled,
    Triggered,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/data/trigger_orders.json");
        let _orders: Vec<super::TriggerOrder> = serde_json::from_str(json).unwrap();
    }
}
