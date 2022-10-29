#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DepositAddress {
    pub address: String,
    pub tag: Option<String>,
}
