use crate::rest::model::coin::Method;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalFee {
    pub fee: Decimal,
    pub method: Method,
    pub congested: bool,
}
