#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    pub coin: String,
    pub free: f64,
    pub total: f64,
    pub spot_borrow: f64,
    pub available_without_borrow: f64,
}
