use crate::rest::model::position::Position;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub backstop_provider: bool,
    pub collateral: Decimal,
    pub free_collateral: Decimal,
    pub initial_margin_requirement: Decimal,
    pub leverage: Decimal,
    pub liquidating: bool,
    pub maintenance_margin_requirement: Decimal,
    pub maker_fee: Decimal,
    pub margin_fraction: Option<Decimal>,
    pub open_margin_fraction: Option<Decimal>,
    pub taker_fee: Decimal,
    pub total_account_value: Decimal,
    pub total_position_size: Decimal,
    pub username: String,
    pub account_identifier: Option<u64>,
    pub positions: Vec<Position>,
}
