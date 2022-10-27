use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub name: String,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub quote_volume_24h: Option<Decimal>,
    pub change_1h: Option<Decimal>,
    pub change_24h: Option<Decimal>,
    pub high_leverage_fee_exempt: bool,
    pub min_provide_size: Option<Decimal>,
    pub r#type: MarketType,
    pub underlying: Option<String>,
    pub enabled: bool,
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub last: Option<Decimal>,
    pub post_only: bool,
    pub price: Option<Decimal>,
    pub price_increment: Option<Decimal>,
    pub size_increment: Option<Decimal>,
    pub restricted: bool,
    pub volume_usd_24h: Option<Decimal>,
    pub large_order_threshold: Option<Decimal>,
    pub is_etf_market: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketType {
    Future,
    Spot,
}
