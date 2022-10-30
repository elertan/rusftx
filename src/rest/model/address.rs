use crate::rest::model::coin::Method;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub address: String,
    pub tag: Option<String>,
    pub coin: Option<String>,
    pub method: Option<Method>,
}
