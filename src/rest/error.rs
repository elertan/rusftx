#[derive(Debug)]
pub enum RestError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Other(String),
}