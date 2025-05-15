use std::sync::{Arc, Mutex};
use config_lib::Settings;
use oauth2_lib::epic::client::{EpicFhirClient};
use oauth2_lib::epic::config::{EpicFhirConfig};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use time::Duration;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub settings: Settings,
    pub epic_client: Mutex<EpicFhirClient>,
}

pub fn session_layer(_settings: &Settings) -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::default();
    SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(600)))
}

/// Khởi tạo state gồm Epic OAuth2 client
pub async fn build_state(settings: Settings) -> anyhow::Result<SharedState> {
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
    let state = AppState { settings, epic_client: Mutex::new(epic_client) };
    Ok(Arc::new(state))
}
