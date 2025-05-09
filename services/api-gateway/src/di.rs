use std::sync::Arc;
use config_lib::Settings;
use oauth2_lib::EpicOAuth2;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use time::Duration;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub settings: Settings,
    pub oauth: EpicOAuth2,
}

pub fn session_layer(settings: &Settings) -> SessionManagerLayer<MemoryStore> {
    let store = MemoryStore::default();
    SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(600)))
}

/// Khởi tạo state gồm OAuth2 client
pub async fn build_state(settings: Settings) -> anyhow::Result<SharedState> {
    let oauth = EpicOAuth2::new(settings.oauth2.clone());
    let state = AppState { settings, oauth };
    Ok(Arc::new(state))
}
