use crate::rest::model::side::Side;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub liquidation: bool,
    pub time: chrono::DateTime<chrono::Utc>,
}
