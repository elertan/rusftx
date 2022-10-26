use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub name: String,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub quote_volume_24h: Option<f64>,
    pub change_1h: Option<f64>,
    pub change_24h: Option<f64>,
    pub high_leverage_fee_exempt: bool,
    pub min_provide_size: Option<f64>,
    pub r#type: MarketType,
    pub underlying: Option<String>,
    pub enabled: bool,
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub last: Option<f64>,
    pub post_only: bool,
    pub price: Option<f64>,
    pub price_increment: Option<f64>,
    pub size_increment: Option<f64>,
    pub restricted: bool,
    pub volume_usd_24h: Option<f64>,
    pub large_order_threshold: Option<f64>,
    pub is_etf_market: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketType {
    Future,
    Spot
}