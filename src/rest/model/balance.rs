use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin: String,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
}

pub type BalancesPerAccount = HashMap<String, Vec<Balance>>;
