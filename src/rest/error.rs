#[derive(Debug)]
pub enum RestError {
    Http(reqwest::Error),
    Other(String),
}