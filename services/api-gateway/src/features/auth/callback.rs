use axum::{extract::{Query, State}, http::StatusCode, response::Redirect};
use serde::Deserialize;
use tower_sessions::Session;
use crate::di::SharedState;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

/// Route /auth/callback sử dụng tower_sessions::Session
pub async fn callback(
    State(state): State<SharedState>,
    session: Session,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, (StatusCode, String)> {
    // Verify state
    let stored: Option<String> = session.get("oauth_state").await.unwrap();
    if stored.as_deref() != Some(&params.state) {
        return Err((StatusCode::BAD_REQUEST, "Invalid state".into()));
    }

    // Đổi code lấy token
    let token = state.oauth.exchange_code(&params.code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Lưu access_token vào session
    session.insert("access_token", token.access_token).await.unwrap();

    // Chuyển hướng về trang chính
    Ok(Redirect::to("/"))
}
