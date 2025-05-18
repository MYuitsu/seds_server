use std::collections::HashMap;
use std::env;
use std::sync::{Arc};
use tokio::sync::Mutex;
use anyhow::Context;
use axum::extract::FromRef;
use axum::response::{IntoResponse, Response};
use config_lib::Settings;
use oauth2::basic::BasicClient;
use oauth2::{ClientId, RedirectUrl};
use oauth2_lib::epic::client::{EpicFhirClient};
use oauth2_lib::epic::config::{EpicFhirConfig};
use reqwest::StatusCode;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use time::Duration;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub settings: Settings,
    pub store: MemoryStore,
    pub oauth_clients: HashMap<String, Arc<Mutex<EpicFhirClient>>>, // Hoặc một kiểu client chung hơn
}

#[derive(Clone)]
pub struct AppMemoryStore(pub MemoryStore); 

#[derive(Clone)]
pub struct AppSettings(pub Settings); // Newtype bao bọc Settings

#[derive(Clone)]
pub struct AppEpicClient(pub Arc<Mutex<EpicFhirClient>>);

pub fn session_layer(_settings: &Settings) -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::default();
    SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(600)))
}

pub async fn build_state(settings: Settings, store: MemoryStore) -> anyhow::Result<SharedState> {
    let mut oauth_clients_map = HashMap::new();

    for (client_name, client_config_values) in &settings.oauth_clients {
        // Chuyển đổi từ OAuth2ClientSettings (trong config_lib) sang EpicFhirConfig (trong oauth2_lib)
        // Hoặc một struct config chung cho client OAuth2 của bạn
        let epic_config = EpicFhirConfig { // Bạn có thể cần một struct config chung hơn nếu các provider khác nhau nhiều
            client_id: client_config_values.client_id.clone(),
            client_secret: client_config_values.client_secret.clone().expect("REASON"),
            auth_url: client_config_values.auth_url.clone(),
            token_url: client_config_values.token_url.clone(),
            redirect_url: client_config_values.redirect_uri.clone(), // Chú ý tên trường
            scopes: client_config_values.scopes.clone(),
            audience: client_config_values.audience.clone(),
            private_key_pem: client_config_values.private_key_pem.clone(),
            key_id: client_config_values.key_id.clone(),
            jwt_algorithm: client_config_values.private_key_algorithm.clone(), // Chú ý tên trường
        };
        
        let client = EpicFhirClient::new(epic_config)
            .map_err(|e| anyhow::anyhow!("Failed to create OAuth client for {}: {:?}", client_name, e))?;
        oauth_clients_map.insert(client_name.clone(), Arc::new(Mutex::new(client)));
    }

    let state = AppState {
        settings,
        store,
        oauth_clients: oauth_clients_map,
    };
    Ok(Arc::new(state))
}

// #[derive(Clone)]
// struct AxumAppState {
//     store: MemoryStore,
//     oauth_client: BasicClient,
// }

// impl FromRef<AxumAppState> for MemoryStore {
//     fn from_ref(state: &AxumAppState) -> Self {
//         state.store.clone()
//     }
// }

// impl FromRef<AxumAppState> for BasicClient {
//     fn from_ref(state: &AxumAppState) -> Self {
//         state.oauth_client.clone()
//     }
// }


#[derive(Debug)]
pub struct AxumAppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AxumAppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AxumAppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn axum_build_state(settings: Settings, store: MemoryStore) -> anyhow::Result<SharedState> {
        let mut oauth_clients_map = HashMap::new();

    for (client_name, client_config_values) in &settings.oauth_clients {
    let epic_config = EpicFhirConfig {
 client_id: client_config_values.client_id.clone(),
            client_secret: client_config_values.client_secret.clone().expect("REASON"),
            auth_url: client_config_values.auth_url.clone(),
            token_url: client_config_values.token_url.clone(),
            redirect_url: client_config_values.redirect_uri.clone(), // Chú ý tên trường
            scopes: client_config_values.scopes.clone(),
            audience: client_config_values.audience.clone(),
            private_key_pem: client_config_values.private_key_pem.clone(),
            key_id: client_config_values.key_id.clone(),
            jwt_algorithm: client_config_values.private_key_algorithm.clone(),
    };
    let client = EpicFhirClient::new(epic_config)
            .map_err(|e| anyhow::anyhow!("Failed to create OAuth client for {}: {:?}", client_name, e))?;
        oauth_clients_map.insert(client_name.clone(), Arc::new(Mutex::new(client)));
    }

    let state = AppState {
        settings,
        store,
        oauth_clients: oauth_clients_map,
    };
    Ok(Arc::new(state))
}

fn axum_oauth_client() -> Result<BasicClient, AxumAppError> {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    Ok(BasicClient::new(
        ClientId::new(client_id)
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
    
}
