use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub start_time: DateTime<Utc>,
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Option<Decimal>,
}
