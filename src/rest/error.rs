#[derive(Debug)]
pub enum RestError {
    Http(reqwest::Error),
    Serde(serde_json::Error),
    Other(String),
}
