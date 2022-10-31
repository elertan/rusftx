use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub order_id: Option<u64>,
    pub order_size: Option<Decimal>,
    pub filled_size: Option<Decimal>,
    pub time: DateTime<Utc>,
    pub error: Option<String>,
}
