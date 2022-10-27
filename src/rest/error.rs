#[derive(Debug)]
pub enum RestError {
    RateLimit,
    Http(reqwest::Error),
    Serde(serde_json::Error),
    Other(String),
}
