mod config;
mod di;
mod features;
mod observability;
mod resilience;
mod routes;

use axum::Router;
use config::load_settings;
use di::SharedState;
use observability::init_tracing;
use time::Duration;
use tokio::net::TcpListener;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // 1. Init tracing/logging
    init_tracing();
    tracing::info!("Starting API Gateway...");

    let store = MemoryStore::default();
    // 2. Load settings
    let settings = load_settings();

    // --- KIỂM TRA CẤU HÌNH OAUTH CLIENTS ---
    tracing::debug!("Loaded Settings: {:#?}", settings);

    if let Some(epic_config_values) = settings.oauth_clients.get("epic_sandbox") {
        tracing::info!("Epic Sandbox OAuth2 Client Configuration Loaded:");
        tracing::info!("  Client ID: {}", epic_config_values.client_id);
        tracing::info!("  Key ID: {:?}", epic_config_values.key_id);
        match &epic_config_values.private_key_pem {
            Some(key_pem) if !key_pem.is_empty() => {
                tracing::info!("  Private Key PEM: Loaded (length: {})", key_pem.len())
            }
            Some(_) => tracing::warn!("  Private Key PEM: Loaded but is an empty string!"),
            None => tracing::warn!("  Private Key PEM: NOT loaded (is None)"),
        }
    } else {
        tracing::error!("Epic Sandbox OAuth2 Client Configuration (epic_sandbox) NOT found in settings.oauth_clients");
    }
    // --- KẾT THÚC KIỂM TRA ---

    // 3. Build application state (di::build_state will use the settings)
    let state: SharedState = di::build_state(settings, store.clone()).await?;

    // Tạo session layer dùng đúng instance store này
    let session_layer = SessionManagerLayer::new(store.clone())
        .with_secure(false) // Để false khi phát triển local, true khi production
        .with_same_site(SameSite::Lax) // hoặc .with_same_site(SameSite::None) nếu cần cross-site
        .with_path("/") // Đảm bảo cookie dùng cho toàn bộ app
        .with_expiry(Expiry::OnInactivity(Duration::seconds(6000)));

    // 4. Build router
    let app = routes::create_router(&state).layer(session_layer);

    // 5. Start server (Axum 0.8+)
    let addr = format!("0.0.0.0:{}", state.settings.port);
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
