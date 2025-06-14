//! Client for handling OAuth2 authentication with Epic FHIR.

use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl,
};

use crate::epic::config::EpicFhirConfig;
use crate::epic::error::Error as EpicError;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Deserialize;
use serde::{Deserialize as SerdeDeserialize, Serialize};
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
    access_token: Option<oauth2::AccessToken>, // Stores the current access token
    refresh_token: Option<oauth2::RefreshToken>, // Stores the current refresh token
    expires_at: Option<Instant>,               // Time when the access token expires
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
    pub fn get_authorization_url(&self) -> Result<(Url, String, String), EpicError> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let csrf_token = CsrfToken::new_random();

        let mut auth_request_builder = self.oauth_client.authorize_url(|| csrf_token.clone());

        for scope_str in &self.config.scopes {
            auth_request_builder = auth_request_builder.add_scope(Scope::new(scope_str.clone()));
        }

        // Epic may require the 'aud' (audience) parameter in the authorization request.
        // This is typically the FHIR server URL.
        auth_request_builder = auth_request_builder.add_extra_param("aud", &self.config.audience);

        let (auth_url, returned_csrf_token) = auth_request_builder
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((
            auth_url,
            returned_csrf_token.secret().clone(),
            pkce_verifier.secret().clone(),
        ))
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
        &self,
        auth_code: String,
        expected_csrf: String,
        expected_pkce: String,
        received_state: String,
    ) -> Result<oauth2::AccessToken, EpicError> {
        // Log toàn bộ cấu hình OAuth2
        tracing::info!("Epic OAuth2 config:");
        tracing::info!("  client_id: {:?}", self.config.client_id);
        tracing::info!("  redirect_url: {:?}", self.config.redirect_url);
        tracing::info!("  token_url: {:?}", self.config.token_url);
        tracing::info!("  auth_url: {:?}", self.config.auth_url);
        tracing::info!("  audience: {:?}", self.config.audience);
        tracing::info!("  scopes: {:?}", self.config.scopes);
        tracing::info!("  private_key_pem: {:?}", self.config.private_key_pem.is_some());
        tracing::info!("  key_id: {:?}", self.config.key_id);
        tracing::info!("  jwt_algorithm: {:?}", self.config.jwt_algorithm);

        // Kiểm tra CSRF
        if expected_csrf != received_state {
            tracing::error!(
                "CSRF mismatch: expected {:?}, received {:?}",
                expected_csrf,
                received_state
            );
            return Err(EpicError::CsrfMismatch);
        }

        let pkce_verifier = PkceCodeVerifier::new(expected_pkce.clone());

        let mut token_request_builder = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(auth_code.clone()))
            .set_pkce_verifier(pkce_verifier);

        // Log các tham số gửi lên token endpoint
        tracing::info!(
            "Token request params:\n  client_id: {:?}\n  redirect_uri: {:?}\n  code: {:?}\n  pkce_verifier: {:?}",
            self.config.client_id,
            self.config.redirect_url,
            auth_code,
            expected_pkce
        );

        // Nếu dùng private_key_jwt, log nội dung JWT assertion và claims
        if let (Some(private_key_pem), Some(key_id), Some(jwt_algorithm_str)) = (
            self.config.private_key_pem.as_ref(),
            self.config.key_id.as_ref(),
            self.config.jwt_algorithm.as_ref(),
        ) {
            let (client_assertion, claims) =
                self.create_client_assertion_jwt_debug(private_key_pem, key_id, jwt_algorithm_str)?;

            tracing::info!(
                "Using private_key_jwt:\n  key_id: {:?}\n  algorithm: {:?}\n  assertion: {}\n  claims: {:#?}",
                key_id,
                jwt_algorithm_str,
                client_assertion,
                claims
            );

            token_request_builder = token_request_builder
                .add_extra_param(
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                )
                .add_extra_param("client_assertion", client_assertion);
        }

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = token_request_builder
            .request_async(&http_client)
            .await
            .map_err(|e| {
                tracing::error!("Epic token exchange error: {:#?}", e);
                EpicError::TokenNotFound
            })?;

        tracing::info!(
            "Token exchange success: access_token={:?}, expires_in={:?}",
            token_result.access_token().secret(),
            token_result.expires_in()
        );

        Ok(token_result.access_token().clone())
    }

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
            Algorithm::ES256 | Algorithm::ES384 => {
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

    /// Tạo JWT assertion và trả về cả claims để debug
    fn create_client_assertion_jwt_debug(
        &self,
        private_key_pem: &str,
        key_id: &str,
        algorithm_str: &str,
    ) -> Result<(String, ClientAssertionClaims), EpicError> {
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
            Algorithm::ES256 | Algorithm::ES384 => {
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

        let expiration = now + (4 * 60 + 50);

        let claims = ClientAssertionClaims {
            iss: self.config.client_id.clone(),
            sub: self.config.client_id.clone(),
            aud: self.config.token_url.clone(),
            jti: Uuid::new_v4().to_string(),
            exp: expiration,
            iat: now,
            nbf: now,
        };

        let mut header = Header::new(algorithm);
        header.kid = Some(key_id.to_string());

        let jwt = encode(&header, &claims, &encoding_key)
            .map_err(|e| EpicError::JwtEncodingError(e.to_string()))?;

        Ok((jwt, claims))
    }
}
