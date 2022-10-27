#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Transfer {
    pub id: u64,
    pub coin: String,
    pub size: f64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub notes: String,
    pub status: String,
}
