//! Client for handling OAuth2 authentication with Epic FHIR.

use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardErrorResponse, StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl
};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicErrorResponseType, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};// Renamed to avoid conflict
use url::Url;
use std::time::{Duration, Instant};
use oauth2::reqwest as reqwest_oauth2;
use crate::epic::config::EpicFhirConfig;
use crate::epic::error::Error as EpicError;

/// An OAuth2 client specifically for Epic FHIR.
///
/// This client handles the OAuth2 Authorization Code Grant with PKCE.
#[derive(Debug)]
pub struct EpicFhirClient {
    config: EpicFhirConfig,
    oauth_client: Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    pkce_verifier: Option<PkceCodeVerifier>,
    csrf_token: Option<CsrfToken>,
    access_token: Option<oauth2::AccessToken>,
    refresh_token: Option<oauth2::RefreshToken>,
    expires_at: Option<Instant>,
}

impl EpicFhirClient {
    /// Creates a new `EpicFhirClient` from the given configuration.
    ///
    /// # Errors
    ///
    /// Returns `EpicError::UrlParse` if any of the URLs in the configuration are invalid.
    pub fn new(config: EpicFhirConfig) -> Result<Self, EpicError> {
        let client_id = ClientId::new(config.client_id.clone());
        // Client secret might not be used for public clients with PKCE,
        // but the oauth2 crate's BasicClient expects it.
        // If Epic's PKCE flow for public clients doesn't use client_secret in token exchange,
        // an empty string might be acceptable, or this needs adjustment based on Epic's spec.
        let client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new(config.auth_url.clone())?;
        let token_url = TokenUrl::new(config.token_url.clone())?;
        let redirect_url = RedirectUrl::new(config.redirect_url.clone())?;

        let oauth_client = BasicClient::new(client_id)
            .set_client_secret(client_secret) // May not be strictly needed for PKCE token exchange with public clients
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url)
            ;

        Ok(Self {
            config,
            oauth_client,
            pkce_verifier: None,
            csrf_token: None,
            access_token: None,
            refresh_token: None,
            expires_at: None,
        })
    }

    /// Generates the authorization URL to redirect the user to.
    ///
    /// This method prepares the PKCE challenge and CSRF token, storing them
    /// internally for later verification.
    ///
    /// # Returns
    ///
    /// A tuple containing the `Url` to redirect the user to and the `String`
    /// secret of the CSRF token (which the calling application should store, e.g., in a session).
    pub fn get_authorization_url(&mut self) -> Result<(Url, String), EpicError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        self.pkce_verifier = Some(pkce_verifier);

        let csrf_token = CsrfToken::new_random();
        self.csrf_token = Some(csrf_token.clone());

        let mut auth_request = self.oauth_client.authorize_url(|| csrf_token.clone());

        for scope_str in &self.config.scopes {
            auth_request = auth_request.add_scope(Scope::new(scope_str.clone()));
        }

        // Epic requires the 'aud' (audience) parameter in the authorization request.
        auth_request = auth_request.add_extra_param("aud", &self.config.audience);

        let (auth_url, returned_csrf_token) = auth_request
            .set_pkce_challenge(pkce_challenge)
            .url();
        
        Ok((auth_url, returned_csrf_token.secret().clone()))
    }

    /// Exchanges an authorization code for an access token.
    ///
    /// This method should be called after the user has been redirected back to
    /// your application's redirect URI with an authorization code and state.
    ///
    /// # Arguments
    ///
    /// * `auth_code`: The authorization code received from Epic.
    /// * `received_state`: The state parameter received from Epic (must match the stored CSRF token secret).
    ///
    /// # Errors
    ///
    /// Returns `EpicError` if the CSRF token doesn't match, PKCE verifier is missing,
    /// or the token exchange fails.
    pub async fn exchange_code_for_token(
        &mut self,
        auth_code: String,
        received_state: String,
    ) -> Result<&oauth2::AccessToken, EpicError> {
        let stored_csrf_secret = self.csrf_token.as_ref().map(|t| t.secret().clone())
            .ok_or_else(|| EpicError::MissingState("CSRF token not set or already used".to_string()))?;

        if stored_csrf_secret != received_state {
            return Err(EpicError::CsrfMismatch);
        }

        let pkce_verifier = self.pkce_verifier.take()
            .ok_or_else(|| EpicError::MissingState("PKCE verifier not set or already used".to_string()))?;

        let oauth2_http_client = reqwest_oauth2::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

        let token_result: StandardTokenResponse<EmptyExtraTokenFields, oauth2::basic::BasicTokenType> = self.oauth_client
            .exchange_code(AuthorizationCode::new(auth_code))
            .set_pkce_verifier(pkce_verifier)
            .request(oauth2_http_client)?;

        self.access_token = Some(token_result.access_token().clone());
        self.refresh_token = token_result.refresh_token();
        if let Some(duration) = token_result.expires_in() {
            self.expires_at = Some(Instant::now() + duration);
        }

        self.csrf_token = None; // Consume CSRF token

        self.access_token.as_ref().ok_or(EpicError::TokenNotFound)
    }

    /// Returns the current access token if available and not expired (basic check).
    ///
    /// TODO: Implement proper token expiration check and refresh logic.
    pub fn get_access_token(&self) -> Option<&oauth2::AccessToken> {
        if let Some(expires_at) = self.expires_at {
            if Instant::now() >= expires_at {
                return None; // Token expired
            }
        }
        self.access_token.as_ref()
    }

    // TODO: Implement `refresh_access_token(&mut self) -> Result<&oauth2::AccessToken, EpicError>`
    // This method would use `self.refresh_token` to get a new access token.
}