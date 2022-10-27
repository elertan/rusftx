use serde::{Deserialize, Serialize};
use crate::rest::model::side::Side;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub filled_size: f64,
    pub future: String,
    pub id: u64,
    pub market: String,
    pub price: f64,
    pub remaining_size: f64,
    pub side: Side,
    pub size: f64,
    pub status: OrderStatus,
    pub r#type: OrderType,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    New,
    Open,
    Closed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit,
    Market,
}
