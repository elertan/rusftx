use crate::rest::model::side::Side;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Fill {
    pub id: u64,
    pub base_currency: Option<String>,
    pub client_order_id: Option<String>,
    pub fee: Decimal,
    pub fee_currency: Option<String>,
    pub fee_rate: Option<Decimal>,
    pub future: Option<String>,
    pub liquidity: Liquidity,
    pub market: String,
    pub order_id: Option<u64>,
    pub price: Decimal,
    pub quote_currency: Option<String>,
    pub side: Side,
    pub size: Decimal,
    pub time: DateTime<Utc>,
    pub r#type: FillType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Liquidity {
    Taker,
    Maker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FillType {
    Order,
    #[serde(rename = "otc")]
    OTC,
    Liquidation,
    Backstop,
    #[serde(rename = "adl")]
    ADL,
    Expiration,
    Unlock,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_from_json() {
        let json = include_str!("../../../tests/data/fills.json");
        let _fills: Vec<super::Fill> = serde_json::from_str(json).unwrap();
    }
}
