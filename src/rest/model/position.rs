use crate::rest::model::side::Side;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub cost: Decimal,
    pub entry_price: Option<Decimal>,
    pub estimated_liquidation_price: Option<Decimal>,
    pub future: String,
    pub initial_margin_requirement: Decimal,
    pub long_order_size: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub net_size: Decimal,
    pub open_size: Decimal,
    pub realized_pnl: Decimal,
    pub recent_average_open_price: Option<Decimal>,
    pub recent_break_even_price: Option<Decimal>,
    pub recent_pnl: Option<Decimal>,
    pub short_order_size: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub unrealized_pnl: Decimal,
    pub collateral_used: Decimal,
}
