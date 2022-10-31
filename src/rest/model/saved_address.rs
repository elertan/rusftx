use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedAddress {
    pub id: u64,
    pub address: String,
    pub coin: String,
    pub name: String,
    pub fiat: bool,
    pub is_primetrust: bool,
    pub last_used_at: DateTime<Utc>,
    pub tag: Option<String>,
    pub whitelisted: Option<bool>,
    pub whitelisted_after: Option<DateTime<Utc>>,
}
