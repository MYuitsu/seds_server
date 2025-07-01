use std::sync::Arc;

use crate::{di::{AppState, SharedState}, features::patientsummary};
use axum::{
    response::Html, // Thêm Html để trả về nội dung HTML đơn giản cho root
    routing::get,
    Router,
};
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer}; // Đảm bảo time crate được import đúng cách

use crate::features::auth;
mod health;
mod jwks;

pub fn create_router(state: &SharedState) -> Router {
    // Đảm bảo SharedState là Arc<AppState>
    Router::new() // Router<()>
        .route("/", get(root_handler)) // Router<()>
        .merge(health::health_routes(state)) // health_routes giờ trả về Router<()>, merge thành công -> Router<()>
        .merge(auth::routes::auth_routes(state)) // Tương tự -> Router<()>
        .merge(jwks::jwks_routes(state)) // Tương tự -> Router<()>// Áp dụng layer, vẫn là Router<()>
                                         // .with_state(state.clone().) // Bây giờ self là Router<()>, state.clone() là Arc<AppState>
                                         // Kết quả sẽ là Router<Arc<AppState>>, khớp với kiểu trả về.
        .merge(patientsummary::routes::patient_summary_routes(state))
}

/// Handler cho root endpoint ("/")
async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome to SEDS API Gateway</h1><p><a href=\"/epic-sandbox/login\">Login with Epic Sandbox</a></p>")
}
