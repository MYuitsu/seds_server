use oauth2::RequestTokenError;
use std::fmt;

#[derive(Debug)]
pub enum OAuth2Error {
    MissingEnv(&'static str),
    OAuth2Request(RequestTokenError<oauth2::reqwest::HttpClientError>),
}

impl fmt::Display for OAuth2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuth2Error::MissingEnv(var) => write!(f, "Missing environment variable: {}", var),
            OAuth2Error::OAuth2Request(e) => write!(f, "OAuth2 request error: {}", e),
        }
    }
}

impl std::error::Error for OAuth2Error {}