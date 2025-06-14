//! Configuration for the Epic FHIR OAuth2 client.

/// Configuration parameters required to connect to Epic FHIR's OAuth2 provider.
#[derive(Debug, Clone)]
pub struct EpicFhirConfig {
    /// The client ID assigned by Epic.
    pub client_id: String,
    /// The client secret assigned by Epic (if applicable for the OAuth2 flow).
    pub client_secret: String,
    /// The Epic OAuth2 authorization endpoint URL.
    /// e.g., "https://fhir.epic.com/interconnect-fhir-oauth/oauth2/authorize"
    pub auth_url: String,
    /// The Epic OAuth2 token endpoint URL.
    /// e.g., "https://fhir.epic.com/interconnect-fhir-oauth/oauth2/token"
    pub token_url: String,
    /// The redirect URI registered with Epic for your application.
    pub redirect_url: String,
    /// A list of scopes your application is requesting.
    /// e.g., vec!["openid", "fhirUser", "patient/*.read", "launch/patient"]
    pub scopes: Vec<String>,
    /// The audience parameter required by Epic, typically the token URL or FHIR server base URL.
    /// e.g., "https://fhir.epic.com/interconnect-fhir-oauth/oauth2/token"
    pub audience: String,
    pub private_key_pem: Option<String>,
    pub key_id: Option<String>,
    pub jwt_algorithm: Option<String>,
}

impl EpicFhirConfig {
    /// Creates a new `EpicFhirConfig`.
    /// All parameters are mandatory as they are essential for the OAuth2 flow with Epic.
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
        scopes: Vec<String>,
        audience: String,
        private_key_pem: Option<String>,
        key_id: Option<String>,
        jwt_algorithm: Option<String>,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_url,
            scopes,
            audience,
            private_key_pem,
            key_id,
            jwt_algorithm,
        }
    }
}
