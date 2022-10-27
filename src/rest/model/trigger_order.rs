use crate::rest::model::order::OrderType;
use crate::rest::model::side::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrder {
    pub id: u64,
    pub market: String,
    pub r#type: TriggerOrderType,
    pub side: Side,
    pub trigger_price: Option<f64>,
    pub order_price: Option<f64>,
    pub size: f64,
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
#[serde(rename_all = "camelCase")]
pub enum TriggerOrderType {
    Stop,
    TrailingStop,
    TakeProfit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderTypeQuery {
    Stop,
    TrailingStop,
    TakeProfit,
}

impl From<TriggerOrderType> for TriggerOrderTypeQuery {
    fn from(t: TriggerOrderType) -> Self {
        match t {
            TriggerOrderType::Stop => Self::Stop,
            TriggerOrderType::TrailingStop => Self::TrailingStop,
            TriggerOrderType::TakeProfit => Self::TakeProfit,
        }
    }
}

impl From<TriggerOrderTypeQuery> for TriggerOrderType {
    fn from(t: TriggerOrderTypeQuery) -> Self {
        match t {
            TriggerOrderTypeQuery::Stop => Self::Stop,
            TriggerOrderTypeQuery::TrailingStop => Self::TrailingStop,
            TriggerOrderTypeQuery::TakeProfit => Self::TakeProfit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerOrderStatus {
    Open,
    Cancelled,
    Triggered,
}
