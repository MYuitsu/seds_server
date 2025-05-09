
use thiserror::Error;

/// Các lỗi có thể xảy ra trong OAuth2 flow
#[derive(Error, Debug)]
pub enum OAuth2Error {
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Unexpected HTTP status: {0}")]
    Status(reqwest::StatusCode),

    #[error("JSON decode error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    Config(String),
}

