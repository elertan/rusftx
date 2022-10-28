use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub name: String,
    pub change_1h: Decimal,
    pub change_24h: Decimal,
    pub change_bod: Decimal,
    pub r#type: FutureType,
    pub underlying: String,
    pub enabled: bool,
    pub expired: bool,
    pub perpetual: bool,
    pub expiry: Option<DateTime<Utc>>,
    pub index: Decimal,
    pub imf_factor: Decimal,
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub last: Option<Decimal>,
    pub lower_bound: Decimal,
    pub upper_bound: Decimal,
    pub mark: Decimal,
    pub open_interest: Decimal,
    pub open_interest_usd: Decimal,
    pub position_limit_weight: Decimal,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub volume_usd_24h: Decimal,
    pub volume: Decimal,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FutureType {
    Future,
    Perpetual,
    Move,
    Prediction,
}
