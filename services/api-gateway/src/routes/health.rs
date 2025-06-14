use crate::di::SharedState;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value}; // Cần thiết nếu bạn muốn truy cập state, dù ở đây không dùng

// Hàm tạo router cho health check
pub fn health_routes(_state: &SharedState) -> Router<()> {
    // _state không được sử dụng ở đây, nhưng giữ signature để nhất quán
    Router::new().route("/health", get(health_check_handler))
    // Không cần .with_state ở đây nếu handler không dùng state
}

// Handler cho health check
async fn health_check_handler() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
