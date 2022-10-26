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
}