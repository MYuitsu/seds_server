use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize)]
struct TokenRequest<'a> {
  grant_type: &'a str,
  code: &'a str,
  redirect_uri: &'a str,
  client_id: &'a str,
  client_secret: &'a str,
}

#[derive(Deserialize)]
pub struct TokenResponse {
  pub access_token: String,
  pub token_type: String,
  pub expires_in: u64,
}

pub struct EpicOAuth2 {
  cfg: crate::config::OAuth2Settings,
  http: Client,
}

impl EpicOAuth2 {
  pub fn new(cfg: crate::config::OAuth2Settings) -> Self {
    Self { cfg, http: Client::new() }
  }

  /// Sinh URL để redirect người dùng lên Epic authorize
  pub fn authorize_url(&self, state: &str) -> String {
    let mut url = Url::parse(&self.cfg.authorize_url).unwrap();
    url.query_pairs_mut()
      .append_pair("response_type", "code")
      .append_pair("client_id", &self.cfg.client_id)
      .append_pair("redirect_uri", &self.cfg.redirect_uri)
      .append_pair("scope", "openid fhirUser")
      .append_pair("state", state);
    url.to_string()
  }

  /// Đổi mã `code` lấy access_token
  pub async fn exchange_code(&self, code: &str) -> anyhow::Result<TokenResponse> {
    let req = TokenRequest {
      grant_type: "authorization_code",
      code,
      redirect_uri: &self.cfg.redirect_uri,
      client_id: &self.cfg.client_id,
      client_secret: &self.cfg.client_secret,
    };
    let resp = self.http
      .post(&self.cfg.token_url)
      .form(&req)
      .send().await?
      .error_for_status()?
      .json::<TokenResponse>().await?;
    Ok(resp)
  }
}
