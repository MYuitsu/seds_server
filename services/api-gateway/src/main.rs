#[tokio::main]
async fn main() {
    let settings = config_lib::Settings::load().unwrap();
    let oauth = oauth2_lib::EpicOAuth2::new(settings.oauth2.clone());
    let session_store = axum_sessions::SessionLayer::new(
        axum_sessions::memory::MemoryStore::new(), 
        &settings.session_key
    );
    let app = Router::new()
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        // ... các route khác ...
        .with_state(AppState { oauth, session: session_store.clone() })
        .layer(session_store);
    // ... run server ...
}
