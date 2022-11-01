use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginMessage {
    pub args: LoginMessageArgs,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginMessageArgs {
    pub key: String,
    pub sign: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub time: DateTime<Utc>,
    pub subaccount: Option<String>,
}
