use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBalancesAndPositionsSnapshot {
    pub id: u64,
    pub accounts: Vec<String>,
    pub time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: HistoricalBalancesAndPositionsSnapshotStatus,
    pub results: Vec<HistoricalBalancesAndPositionsSnapshotResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoricalBalancesAndPositionsSnapshotStatus {
    Requested,
    Processing,
    Done,
    Cancelled,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalBalancesAndPositionsSnapshotResult {
    pub account: String,
    pub ticker: String,
    pub size: Decimal,
    pub price: Option<Decimal>,
}
