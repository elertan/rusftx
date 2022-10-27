use crate::rest::model::side::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: u64,
    pub client_id: Option<String>,
    pub market: String,
    pub r#type: OrderType,
    pub side: Side,
    pub price: Option<f64>,
    pub size: f64,
    pub status: OrderStatus,
    pub filled_size: f64,
    pub remaining_size: f64,
    pub reduce_only: bool,
    pub liquidation: Option<bool>,
    pub avg_fill_price: Option<f64>,
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
