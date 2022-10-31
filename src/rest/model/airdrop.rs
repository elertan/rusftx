use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airdrop {
    pub id: u64,
    pub coin: String,
    pub size: Decimal,
    pub status: AirdropStatus,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AirdropStatus {
    Confirmed,
    Pending,
}
