mod client;
mod error;
mod types;

pub use client::EpicFhirOAuth2;
pub use error::OAuth2Error;
pub use types::{OAuth2Token, OAuth2TokenResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use oauth2::PkceCodeVerifier;

    #[tokio::test]
    async fn test_authorize_url() {
        let client = EpicFhirOAuth2::new_mock();
        let (url, _csrf, _pkce_challenge, _pkce_verifier) = client.authorize_url(vec!["openid".to_string()]);
        assert!(url.contains("authorize"));
    }

    #[tokio::test]
    async fn test_exchange_code() {
        let client = EpicFhirOAuth2::new_mock();
        let code = "dummy_code".to_string();
        let pkce_verifier = PkceCodeVerifier::new("dummy_verifier".to_string());
        let result = client.exchange_code(code, pkce_verifier).await;
        assert!(result.is_err()); // Vì đây là mock, luôn lỗi
    }
}