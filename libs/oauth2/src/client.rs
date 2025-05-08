use crate::error::OAuth2Error;
use crate::types::{OAuth2Token, OAuth2TokenResponse};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse,
    TokenUrl,
};
use std::env;

pub struct EpicFhirOAuth2 {
    client: BasicClient,
}

impl EpicFhirOAuth2 {
    /// Khởi tạo client với thông tin từ biến môi trường
    pub fn new() -> Result<Self, OAuth2Error> {
        let client_id = ClientId::new(env::var("EPIC_CLIENT_ID").map_err(|_| OAuth2Error::MissingEnv("EPIC_CLIENT_ID"))?);
        let client_secret = ClientSecret::new(env::var("EPIC_CLIENT_SECRET").map_err(|_| OAuth2Error::MissingEnv("EPIC_CLIENT_SECRET"))?);
        let auth_url = AuthUrl::new("https://fhir.epic.com/interconnect-fhir-oauth/oauth2/authorize".to_string()).unwrap();
        let token_url = TokenUrl::new("https://fhir.epic.com/interconnect-fhir-oauth/oauth2/token".to_string()).unwrap();
        let redirect_url = RedirectUrl::new(env::var("EPIC_REDIRECT_URI").map_err(|_| OAuth2Error::MissingEnv("EPIC_REDIRECT_URI"))?).unwrap();

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url);

        Ok(Self { client })
    }

    /// Sinh URL để redirect người dùng đăng nhập Epic FHIR, trả về (url, csrf_token, pkce_challenge, pkce_verifier)
    pub fn authorize_url(&self, scopes: Vec<String>) -> (String, CsrfToken, PkceCodeChallenge, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let mut auth_req = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge.clone());

        for scope in scopes {
            auth_req = auth_req.add_scope(Scope::new(scope));
        }

        let (auth_url, csrf_token) = auth_req.url();

        (auth_url.to_string(), csrf_token, pkce_challenge, pkce_verifier)
    }

    /// Đổi code lấy access token
    pub async fn exchange_code(
        &self,
        code: String,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<OAuth2Token, OAuth2Error> {
        let token_result = self.client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(OAuth2Error::OAuth2Request)?;

        Ok(OAuth2Token::from(token_result))
    }
}