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
    pub epic_client: Mutex<EpicFhirClient>,
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

/// Khởi tạo state gồm Epic OAuth2 client
pub async fn build_state(settings: Settings, store: MemoryStore) -> anyhow::Result<SharedState> {
    // Chuyển đổi từ Settings sang EpicFhirConfig
    let epic_config = EpicFhirConfig {
        client_id: settings.oauth2.client_id.clone(),
        client_secret: settings.oauth2.client_secret.clone(),
        auth_url: settings.oauth2.auth_url.clone(),
        token_url: settings.oauth2.token_url.clone(),
        redirect_url: settings.oauth2.redirect_uri.clone(),
        scopes: settings.oauth2.scopes.clone(),
        audience: settings.oauth2.audience.clone(),
    };
    let epic_client = EpicFhirClient::new(epic_config)?;
    let state = AppState { settings, store, epic_client: Mutex::new(epic_client) };
    Ok(Arc::new(state))
}


#[derive(Clone)]
struct AxumAppState {
    store: MemoryStore,
    oauth_client: BasicClient,
}

impl FromRef<AxumAppState> for MemoryStore {
    fn from_ref(state: &AxumAppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AxumAppState> for BasicClient {
    fn from_ref(state: &AxumAppState) -> Self {
        state.oauth_client.clone()
    }
}
#[derive(Debug)]
struct AxumAppError(anyhow::Error);

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
    // Chuyển đổi từ Settings sang EpicFhirConfig
    let epic_config = EpicFhirConfig {
        client_id: settings.oauth2.client_id.clone(),
        client_secret: settings.oauth2.client_secret.clone(),
        auth_url: settings.oauth2.auth_url.clone(),
        token_url: settings.oauth2.token_url.clone(),
        redirect_url: settings.oauth2.redirect_uri.clone(),
        scopes: settings.oauth2.scopes.clone(),
        audience: settings.oauth2.audience.clone(),
    };
    let epic_client = EpicFhirClient::new(epic_config)?;
    let state = AppState { settings, store, epic_client: Mutex::new(epic_client) };
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
