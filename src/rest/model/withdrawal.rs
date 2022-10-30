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
    // pub address: Option<Address>,
    // pub uploaded_file: Option<String>,
    // pub uploaded_file_name: Option<String>,
    // pub cancel_reason: Option<String>,
    // pub fiat: Option<bool>,
    // pub ach: Option<bool>,
    // pub r#type: Option<String>,
    // pub support_ticket_id: Option<u64>,
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
