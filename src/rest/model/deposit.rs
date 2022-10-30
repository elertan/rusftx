use crate::rest::model::address::Address;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// {
// "id": 30984052,
// "coin": "USDC",
// "txid": "0xe01dcb1a7c2fcb9511e8a7e381289ec1c23d04bc9e6bb6fdfb411d44ad021d6a",
// "address": {
// "address": "0xa72A857F2Bd89BaD0A43E24C9236A9e9eA3A6104",
// "tag": null,
// "method": "erc20",
// "coin": null
// },
// "size": 9003.0,
// "fee": 0.0,
// "status": "confirmed",
// "time": "2022-10-30T00:05:23.004410+00:00",
// "sentTime": "2022-10-30T00:05:23.013506+00:00",
// "confirmedTime": "2022-10-30T00:05:36.049758+00:00",
// "confirmations": 17,
// "method": "avax"
// },

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub id: u64,
    pub coin: String,
    pub confirmations: Option<u64>,
    pub confirmed_time: Option<DateTime<Utc>>,
    pub fee: Option<Decimal>,
    pub sent_time: Option<DateTime<Utc>>,
    pub size: Decimal,
    pub status: DepositStatus,
    pub time: DateTime<Utc>,
    pub txid: Option<String>,
    pub notes: Option<String>,
    pub address: Option<Address>,
    pub uploaded_file: Option<String>,
    pub uploaded_file_name: Option<String>,
    pub cancel_reason: Option<String>,
    pub fiat: Option<bool>,
    pub ach: Option<bool>,
    pub r#type: Option<String>,
    pub support_ticket_id: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DepositStatus {
    Complete,
    Confirmed,
    Unconfirmed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize() {
        let json = include_str!("../../../tests/data/deposits.json");
        let _deposits: Vec<super::Deposit> = serde_json::from_str(json).unwrap();
    }
}
