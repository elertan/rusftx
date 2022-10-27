use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    pub coin: String,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
}
