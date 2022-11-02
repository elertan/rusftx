use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub name: String,
    pub change_1h: Option<Decimal>,
    pub change_24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub r#type: FutureType,
    pub underlying: String,
    pub enabled: bool,
    pub expired: bool,
    pub perpetual: bool,
    pub expiry: Option<DateTime<Utc>>,
    pub index: Option<Decimal>,
    pub imf_factor: Decimal,
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub last: Option<Decimal>,
    pub lower_bound: Option<Decimal>,
    pub upper_bound: Option<Decimal>,
    pub mark: Option<Decimal>,
    pub open_interest: Option<Decimal>,
    pub open_interest_usd: Option<Decimal>,
    pub position_limit_weight: Decimal,
    pub post_only: bool,
    pub price_increment: Option<Decimal>,
    pub size_increment: Option<Decimal>,
    pub volume_usd_24h: Option<Decimal>,
    pub volume: Option<Decimal>,
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
