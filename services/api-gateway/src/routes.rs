use axum::{routing::{any, get}, Router};
use crate::di::SharedState;
use crate::features::auth::callback::callback;
use crate::features::auth::login::login;
use tower_sessions::{MemoryStore, SessionManagerLayer, Expiry};
use time::Duration;

/// Tạo router chính
pub fn create_router(state: &SharedState) -> Router {
    let session_layer = SessionManagerLayer::new(MemoryStore::default())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(600)));

    Router::new()
        .route("/auth/login", any(<dyn axum::handler::Handler<(), _>>::from_fn(login)))
        .route("/auth/callback", any(callback))
        .route("/", get(root))
        .layer(session_layer)
        .with_state(state.clone())
}

/// Root endpoint
async fn root() -> &'static str {
    "Welcome to API Gateway"
}
