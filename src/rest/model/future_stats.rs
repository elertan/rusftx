use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureStats {
    pub volume: Decimal,
    pub next_funding_rate: Option<Decimal>,
    pub next_funding_time: Option<DateTime<Utc>>,
    pub expiration_price: Option<Decimal>,
    pub predicted_expiration_price: Option<Decimal>,
    pub open_interest: Decimal,
    pub strike_price: Option<Decimal>,
}
