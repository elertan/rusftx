#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Orderbook {
    pub asks: Vec<[f64; 2]>,
    pub bids: Vec<[f64; 2]>,
}
