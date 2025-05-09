use axum::{extract::State, response::Redirect};
use tower_sessions::Session;
use uuid::Uuid;
use crate::di::SharedState;

/// Route /auth/login sử dụng tower_sessions::Session
pub async fn login(
    State(state): State<SharedState>,
    session: Session,
) -> Redirect {
    // Tạo state ngẫu nhiên, lưu vào session
    let state_token = Uuid::new_v4().to_string();
    session.insert("oauth_state", state_token.clone()).await.unwrap();

    // Redirect người dùng lên Epic authorize URL
    let url = state.oauth.authorize_url(&state_token)
        .expect("Invalid authorize URL");
    Redirect::to(&url)
}