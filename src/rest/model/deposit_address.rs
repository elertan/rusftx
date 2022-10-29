#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DepositAddress {
    pub address: String,
    pub tag: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DepositAddressWithCoin {
    pub coin: String,
    #[serde(flatten)]
    pub deposit_address: DepositAddress,
}
