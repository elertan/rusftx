#[derive(Debug)]
pub enum RestError {
    RateLimit,
    Http(reqwest::Error),
    Serde(serde_json::Error),
    Urlencode(serde_urlencoded::ser::Error),
    Other(String),
}
