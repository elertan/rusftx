use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Orderbook {
    pub asks: Vec<[Decimal; 2]>,
    pub bids: Vec<[Decimal; 2]>,
}
