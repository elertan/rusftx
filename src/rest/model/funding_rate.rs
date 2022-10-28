use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingRate {
    pub future: String,
    pub rate: Decimal,
    pub time: DateTime<Utc>,
}
