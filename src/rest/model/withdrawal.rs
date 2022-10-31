use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub id: u64,
    pub coin: String,
    pub fee: Option<Decimal>,
    pub size: Decimal,
    pub status: WithdrawalStatus,
    pub time: DateTime<Utc>,
    pub txid: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WithdrawalStatus {
    Requested,
    Processing,
    Sent,
    Complete,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    Wormhole,
    Sollet,
}
