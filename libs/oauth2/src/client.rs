
use crate::{error::OAuth2Error, types::TokenResponse};
use config_lib::settings::OAuth2Settings;
use reqwest::Client;
use url::Url;

/// Client OAuth2 cho Epic SMART on FHIR
pub struct EpicOAuth2 {
    cfg: OAuth2Settings,
    http: Client,
}

impl EpicOAuth2 {
    /// Khởi tạo client với cấu hình từ libs/config
    pub fn new(cfg: OAuth2Settings) -> Self {
        Self {
            cfg,
            http: Client::new(),
        }
    }

    /// Tạo URL để redirect người dùng đến Epic authorize endpoint
    pub fn authorize_url(&self, state: &str) -> Result<String, OAuth2Error> {
        let mut url = Url::parse(&self.cfg.authorize_url)?;
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.cfg.client_id)
            .append_pair("redirect_uri", &self.cfg.redirect_uri)
            .append_pair("scope", "openid fhirUser")
            .append_pair("state", state);
        Ok(url.into())
    }

    /// Đổi authorization code lấy access token
    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse, OAuth2Error> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.cfg.redirect_uri),
            ("client_id", &self.cfg.client_id),
            ("client_secret", &self.cfg.client_secret),
        ];

        let resp = self
            .http
            .post(&self.cfg.token_url)
            .form(&params)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(OAuth2Error::Status(resp.status()));
        }

        let token = resp.json::<TokenResponse>().await?;
        Ok(token)
    }
}
