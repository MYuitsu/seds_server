use axum::{extract::State, response::Redirect};
use tower_sessions::Session;
use crate::di::SharedState;

/// Route /auth/login sử dụng tower_sessions::Session
pub async fn login(
    State(state): State<SharedState>,
    session: Session,
) -> Redirect {
    // Lấy mutable EpicFhirClient từ state (giả sử bạn dùng Arc<Mutex<...>> nếu cần)
    let mut epic_client = state.epic_client.lock().unwrap();

    // Lấy URL và CSRF token
    let (auth_url, csrf_token) = epic_client.get_authorization_url().expect("Failed to build Epic authorize URL");

    // Lưu CSRF vào session để kiểm tra ở callback
    session.insert("oauth_state", csrf_token).await.unwrap();

    Redirect::to(auth_url.as_str())
}