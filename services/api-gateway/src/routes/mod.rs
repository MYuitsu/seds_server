use std::sync::Arc;

use axum::{
    routing::get,
    Router,
    response::Html, // Thêm Html để trả về nội dung HTML đơn giản cho root
};
use crate::di::{AppState, SharedState};
use tower_sessions::{MemoryStore, SessionManagerLayer, Expiry};
use time::Duration; // Đảm bảo time crate được import đúng cách

use crate::features::auth;
mod health;
mod jwks;

pub fn create_router(state: &SharedState) -> Router { // Đảm bảo SharedState là Arc<AppState>
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::seconds(3600))); // Sử dụng time::Duration nếu bạn dùng `time` crate
                                                                        // hoặc std::time::Duration::from_secs(3600)

    Router::new() // Router<()>
        .route("/", get(root_handler)) // Router<()>
        // .merge(health::health_routes(state)) // health_routes giờ trả về Router<()>, merge thành công -> Router<()>
        // .merge(auth::routes::auth_routes(state)) // Tương tự -> Router<()>
        // .merge(jwks::jwks_routes(state)) // Tương tự -> Router<()>
        // .layer(session_layer) // Áp dụng layer, vẫn là Router<()>
        .with_state(state.clone().) // Bây giờ self là Router<()>, state.clone() là Arc<AppState>
                                   // Kết quả sẽ là Router<Arc<AppState>>, khớp với kiểu trả về.
}

/// Handler cho root endpoint ("/")
async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome to SEDS API Gateway</h1><p><a href=\"/epic-sandbox/login\">Login with Epic Sandbox</a></p>")
}
