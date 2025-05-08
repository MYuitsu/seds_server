use oauth2::StandardTokenResponse;
use oauth2::EmptyExtraTokenFields;
use oauth2::basic::BasicTokenType;

#[derive(Debug, Clone)]
pub struct OAuth2Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: String,
}

pub type OAuth2TokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

impl From<OAuth2TokenResponse> for OAuth2Token {
    fn from(resp: OAuth2TokenResponse) -> Self {
        OAuth2Token {
            access_token: resp.access_token().secret().to_string(),
            refresh_token: resp.refresh_token().map(|t| t.secret().to_string()),
            expires_in: resp.expires_in().map(|v| v.as_secs()),
            token_type: resp.token_type().as_str().to_string(),
        }
    }
}