#[cfg(test)]
mod tests {
    use super::*;
    use config_lib::OAuth2Settings;

    #[tokio::test]
    async fn test_authorize_url() {
        let cfg = OAuth2Settings {
            client_id: "cid".into(),
            client_secret: "secret".into(),
            authorize_url: "https://example.com/authorize".into(),
            token_url: "https://example.com/token".into(),
            redirect_uri: "https://app/callback".into(),
        };
        let client = EpicOAuth2::new(cfg);
        let url = client.authorize_url("state123").unwrap();
        assert!(url.contains("response_type=code"));
        assert!(url.contains("state=state123"));
    }
}
