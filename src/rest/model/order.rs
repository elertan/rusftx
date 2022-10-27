use crate::rest::model::side::Side;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: u64,
    pub client_id: Option<String>,
    pub market: String,
    pub r#type: OrderType,
    pub side: Side,
    pub price: Option<Decimal>,
    pub size: Decimal,
    pub status: OrderStatus,
    pub filled_size: Decimal,
    pub remaining_size: Decimal,
    pub reduce_only: bool,
    pub liquidation: Option<bool>,
    pub avg_fill_price: Option<Decimal>,
    pub post_only: bool,
    pub ioc: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub future: Option<String>,
    pub twap_order_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    New,
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit,
    Market,
}
