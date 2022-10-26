use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    name: String,
    base_currency: Option<String>,
    quote_currency: Option<String>,
    quote_volume_24h: Option<f64>,
    change_1h: Option<f64>,
    change_24h: Option<f64>,
    high_leverage_fee_exempt: bool,
    min_provide_size: Option<f64>,
    r#type: MarketType,
    underlying: Option<String>,
    enabled: bool,
    ask: Option<f64>,
    bid: Option<f64>,
    last: Option<f64>,
    post_only: bool,
    price: Option<f64>,
    price_increment: Option<f64>,
    size_increment: Option<f64>,
    restricted: bool,
    volume_usd_24h: Option<f64>,
    large_order_threshold: Option<f64>,
    is_etf_market: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketType {
    Future,
    Spot
}