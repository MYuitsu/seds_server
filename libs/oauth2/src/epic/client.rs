//! Client for handling OAuth2 authentication with Epic FHIR.

use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl
};
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse,
};

use crate::epic::config::EpicFhirConfig;
use crate::epic::error::Error as EpicError;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize as SerdeDeserialize, Serialize};
use serde::Deserialize;
use super::error::AxumAppError;
use std::time::Instant;
use url::Url;
use uuid::Uuid;
/// An OAuth2 client specifically for Epic FHIR.
///
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
    access_token: Option<oauth2::AccessToken>, // Stores the current access token
    refresh_token: Option<oauth2::RefreshToken>, // Stores the current refresh token
    expires_at: Option<Instant>,                 // Time when the access token expires
}

/// Claims for the JWT used in `private_key_jwt` client authentication.
#[derive(Debug, Serialize, SerdeDeserialize)] // Use SerdeDeserialize to avoid conflict
struct ClientAssertionClaims {
    iss: String, // Issuer: client_id
    sub: String, // Subject: client_id
    aud: String, // Audience: token endpoint URL
    jti: String, // JWT ID: unique identifier
    exp: u64,    // Expiration Time: timestamp (seconds since epoch)
    iat: u64,    // Issued At: timestamp (seconds since epoch)
    nbf: u64,    // Not Before: timestamp (seconds since epoch)
}

impl EpicFhirClient {
    /// Creates a new `EpicFhirClient` from the given configuration.
    ///
    /// # Errors
    ///
    /// Returns `EpicError::UrlParse` if any of the URLs in the configuration are invalid.
    pub fn new(config: EpicFhirConfig) -> Result<Self, EpicError> {
        let client_id = ClientId::new(config.client_id.clone());
        let auth_url = AuthUrl::new(config.auth_url.clone())?;
        let token_url = TokenUrl::new(config.token_url.clone())?;
        let redirect_url = RedirectUrl::new(config.redirect_url.clone())?;

        let mut oauth_client_builder = BasicClient::new(client_id)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(redirect_url);

        // If not using private_key_jwt (indicated by presence of private_key_pem and key_id)
        // AND a client_secret is provided, configure the BasicClient to use it.
        // This will typically result in HTTP Basic Authentication for the token endpoint.
        if config.private_key_pem.is_none() || config.key_id.is_none() {
            // Based on the compiler error, config.client_secret is String, not Option<String>.
            // We check if the client_secret string is not empty before using it.
            if !config.client_secret.is_empty() {
                oauth_client_builder = oauth_client_builder
                    .set_client_secret(ClientSecret::new(config.client_secret.clone()));
            }
        }
        // If private_key_jwt is used, client_secret on BasicClient should NOT be set,
        // as we'll provide client_assertion via extra_params.

        let oauth_client = oauth_client_builder;

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

        let mut auth_request_builder = self.oauth_client.authorize_url(|| csrf_token.clone());

        for scope_str in &self.config.scopes {
            auth_request_builder =
                auth_request_builder.add_scope(Scope::new(scope_str.clone()));
        }

        // Epic may require the 'aud' (audience) parameter in the authorization request.
        // This is typically the FHIR server URL.
        auth_request_builder =
            auth_request_builder.add_extra_param("aud", &self.config.audience);

        let (auth_url, returned_csrf_token) = auth_request_builder
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((auth_url, returned_csrf_token.secret().clone()))
    }

    /// Exchanges an authorization code for an access token.
    ///
    /// This method should be called after the user has been redirected back to
    /// your application's redirect URI with an authorization code and state.
    /// It handles client authentication using `private_key_jwt` if configured,
    /// otherwise relies on the `BasicClient`'s configured authentication (e.g., client_secret via Basic Auth, or public client).
    ///
    /// # Arguments
    ///
    /// * `auth_code`: The authorization code received from Epic.
    /// * `received_state`: The state parameter received from Epic (must match the stored CSRF token secret).
    ///
    /// # Errors
    ///
    /// Returns `EpicError` if CSRF/PKCE validation fails or the token exchange fails.
    pub async fn exchange_code(
        &mut self,
        auth_code: String,
        received_state: String,
    ) -> Result<oauth2::AccessToken, EpicError> {
        let stored_csrf_secret = self
            .csrf_token
            .as_ref()
            .map(|t| t.secret().clone())
            .ok_or_else(|| {
                EpicError::MissingState("CSRF token not set or already used".to_string())
            })?;

        if stored_csrf_secret != received_state {
            return Err(EpicError::CsrfMismatch);
        }

        let pkce_verifier = self.pkce_verifier.take().ok_or_else(|| {
            EpicError::MissingState("PKCE verifier not set or already used".to_string())
        })?;

        let mut token_request_builder = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(auth_code))
            .set_pkce_verifier(pkce_verifier);

        // Client Authentication Logic:
        // If private_key_jwt is configured, use it.
        // Otherwise, BasicClient will use its configured auth (e.g., client_secret via Basic Auth, or public client).
        if let (Some(private_key_pem), Some(key_id), Some(jwt_algorithm_str)) = (
            self.config.private_key_pem.as_ref(),
            self.config.key_id.as_ref(),
            self.config.jwt_algorithm.as_ref(),
        ) {
            // Ensure BasicClient was not configured with a client_secret if using private_key_jwt
            if !self.config.client_secret.is_empty() && (self.config.private_key_pem.is_some() && self.config.key_id.is_some()) {
                // This scenario is ambiguous: both client_secret and private_key_jwt are configured.
                // Preferring private_key_jwt as it's generally more secure for backend auth.
                // Consider logging a warning or making this an explicit configuration error.
            }

            let client_assertion =
                self.create_client_assertion_jwt(private_key_pem, key_id, jwt_algorithm_str)?;

            token_request_builder = token_request_builder
                .add_extra_param(
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                )
                .add_extra_param("client_assertion", client_assertion);
            // client_id is typically sent by BasicClient by default or included in the assertion.
            // If Epic specifically requires client_id as a separate param even with private_key_jwt, add it:
            // token_request_builder = token_request_builder.add_extra_param("client_id", self.config.client_id.clone());
        }
        // Else: self.oauth_client will use its pre-configured authentication method
        // (e.g., HTTP Basic Auth if client_secret was set, or just client_id for public clients).
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");
        let token_result = token_request_builder
            .request_async(&http_client) // Use the directly imported function
            .await
            .map_err(|e| {
                // Provide more context from the error if possible
                let error_detail = match e {
                    oauth2::RequestTokenError::ServerResponse(err_resp) => format!(
                        "Server error: {:?}, description: {:?}, URI: {:?}",
                        err_resp.error(),
                        err_resp.error_description(),
                        err_resp.error_uri()
                    ),
                    oauth2::RequestTokenError::Request(req_err) => {
                        format!("Request error: {}", req_err)
                    }
                    oauth2::RequestTokenError::Parse(parse_err, ref body) => {
                        let body_str = String::from_utf8_lossy(body);
                        format!("Parse error: {}, body: {}", parse_err, body_str)
                    }
                    oauth2::RequestTokenError::Other(msg) => format!("Other error: {}", msg),
                };
                EpicError::TokenNotFound
            })?;

        self.access_token = Some(token_result.access_token().clone());
        self.refresh_token = token_result.refresh_token().cloned();
        if let Some(duration) = token_result.expires_in() {
            self.expires_at = Some(Instant::now() + duration);
        } else {
            self.expires_at = None; // Or handle as an error / default short expiry
        }

        self.csrf_token = None; // Consume CSRF token
        self.pkce_verifier = None; // Consume PKCE verifier

        Ok(token_result.access_token().clone())
    }

    /// Returns a clone of the current access token if available and not expired.
    pub fn get_access_token(&self) -> Option<oauth2::AccessToken> {
        if let Some(expires_at) = self.expires_at {
            if Instant::now() >= expires_at {
                return None; // Token expired
            }
        }
        self.access_token.clone()
    }

    // TODO: Implement `refresh_access_token(&mut self) -> Result<oauth2::AccessToken, EpicError>`
    // This method would use `self.refresh_token` to get a new access token.
    // It should also handle client authentication (private_key_jwt or client_secret)
    // similar to the `exchange_code` method.

    /// Creates a signed JWT for `private_key_jwt` client authentication.
    fn create_client_assertion_jwt(
        &self,
        private_key_pem: &str,
        key_id: &str,
        algorithm_str: &str,
    ) -> Result<String, EpicError> {
        let algorithm = match algorithm_str.to_uppercase().as_str() {
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            _ => return Err(EpicError::Other("Unsupported JWT algorithm".to_string())),
        };

        let encoding_key = match algorithm {
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
                    .map_err(|e| EpicError::JwtKeyError(e.to_string()))?
            }
            Algorithm::ES256 | Algorithm::ES384 | Algorithm::ES256 => {
                EncodingKey::from_ec_pem(private_key_pem.as_bytes())
                    .map_err(|e| EpicError::JwtKeyError(e.to_string()))?
            }
            _ => {
                return Err(EpicError::JwtKeyError(
                    "Algorithm not supported for PEM key type".to_string(),
                ))
            }
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| EpicError::TimeError(e.to_string()))?
            .as_secs();

        // JWT `exp` claim should be no more than 5 minutes in the future.
        let expiration = now + (4 * 60 + 50); // Approx 4m50s to be safe.

        let claims = ClientAssertionClaims {
            iss: self.config.client_id.clone(),
            sub: self.config.client_id.clone(),
            aud: self.config.token_url.clone(), // Audience is the token endpoint URL
            jti: Uuid::new_v4().to_string(),    // Unique token ID
            exp: expiration,
            iat: now,
            nbf: now, // Not before, can be same as iat
        };

        let mut header = Header::new(algorithm);
        header.kid = Some(key_id.to_string()); // Key ID for JWKS lookup

        encode(&header, &claims, &encoding_key)
            .map_err(|e| EpicError::JwtEncodingError(e.to_string()))
    }
}