use crate::rest::model::side::Side;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: f64,
    pub size: f64,
    pub side: Side,
    pub liquidation: bool,
    pub time: chrono::DateTime<chrono::Utc>,
}
