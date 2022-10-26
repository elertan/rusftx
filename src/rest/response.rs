use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RestResponse<T> {
    Ok(OkResponse<T>),
    Error(ErrorResponse),
}

#[derive(Debug, Clone, Deserialize)]
pub struct OkResponse<T> {
    pub success: bool,
    pub result: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}