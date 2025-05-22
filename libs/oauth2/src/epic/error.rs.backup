//! Error types for the Epic FHIR OAuth2 client.

use axum::response::{IntoResponse, Response};
use oauth2::{
    HttpClientError, RequestTokenError, StandardErrorResponse,
    basic::{BasicErrorResponse, BasicErrorResponseType},
};
use oauth2::reqwest::Error as OAuth2ReqwestError; // Alias for clarity
use oauth2::url;
use reqwest::StatusCode;

/// Represents errors that can occur during Epic FHIR OAuth2 interactions.
#[derive(Debug)]
pub enum Error {
    /// Error parsing a URL.
    UrlParse(url::ParseError),
    /// Generic OAuth2 error, often from the `oauth2` crate.
    /// It's recommended to make this more specific if possible,
    /// e.g., by wrapping `RequestTokenError` directly.
    OAuth2(String),
    /// Error from the underlying HTTP client (reqwest).
    Reqwest(reqwest::Error),
    /// A required state (like PKCE verifier or CSRF token) was missing.
    MissingState(String),
    /// Access token was expected but not found.
    TokenNotFound,
    /// CSRF token mismatch during the OAuth2 flow.
    CsrfMismatch,
    /// Specific error during the token exchange phase.
    TokenExchange(
        RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>
    ),
    Other(String),
    JwtKeyError(String),
    JwtEncodingError(String),
    TimeError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParse(e) => write!(f, "URL parsing error: {}", e),
            Error::OAuth2(s) => write!(f, "OAuth2 error: {}", s),
            Error::Reqwest(e) => write!(f, "HTTP request error: {}", e),
            Error::MissingState(s) => write!(f, "Missing state: {}", s),
            Error::TokenNotFound => write!(f, "Access token not found"),
            Error::CsrfMismatch => write!(f, "CSRF token mismatch"),
            Error::TokenExchange(e) => write!(f, "Token HTTP client error: {}", e),
            Error::Other(s) => write!(f, "Other error: {}", s),
            Error::JwtKeyError(s) => write!(f, "JWT key error: {}", s),
            Error::JwtEncodingError(s) => write!(f, "JWT encoding error: {}", s),
            Error::TimeError(s) => write!(f, "Time error: {}", s)

        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UrlParse(e) => Some(e),
            Error::Reqwest(e) => Some(e),
            Error::TokenExchange(e) => Some(e),
            _ => None,
        }
    }
}

impl From<url::ParseError> for Error { fn from(err: url::ParseError) -> Self { Error::UrlParse(err) } }
impl From<reqwest::Error> for Error { fn from(err: reqwest::Error) -> Self { Error::Reqwest(err) } }

impl From<RequestTokenError<reqwest::Error, BasicErrorResponse>> for Error {
    fn from(err: RequestTokenError<reqwest::Error, BasicErrorResponse>) -> Self {
        // Chuyển đổi sang HttpClientError nếu cần, hoặc wrap lại bằng variant khác
        Error::Other(format!("{err:?}"))
    }
}

impl From<RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>> for Error {
    fn from(err: RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
        Error::TokenExchange(err)
    }
}
#[derive(Debug)]
pub struct AxumAppError {
    error: anyhow::Error,
    status_code: StatusCode,
}
impl AxumAppError {
    // Constructor to create AxumAppError from a StatusCode and a message
    pub fn new(status_code: StatusCode, message: String) -> Self {
        Self {
            error: anyhow::anyhow!(message),
            status_code,
        }
    }
}
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AxumAppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.error);

        // Use the stored status code
        (self.status_code, format!("Error: {}", self.error.root_cause())).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AxumAppError>` with a default status code (e.g., INTERNAL_SERVER_ERROR).
impl<E> From<E> for AxumAppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            error: err.into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR, // Default status code
        }
    }
}
