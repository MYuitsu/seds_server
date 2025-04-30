use axum::{extract::{Query, State, TypedHeader}, response::Redirect};
use serde::Deserialize;
use http::header::SET_COOKIE;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

pub async fn callback(
    State(state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    Query(params): Query<CallbackParams>,
) -> Result<Redirect, (StatusCode, String)> {
    // 1. Verify state
    let stored: Option<String> = state.session.get("oauth_state").unwrap();
    if stored.as_deref() != Some(&params.state) {
        return Err((StatusCode::BAD_REQUEST, "Invalid state".into()));
    }
    // 2. Exchange code lấy token
    let token_resp = state.oauth.exchange_code(&params.code).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // 3. Lưu access_token vào session hoặc cookie
    state.session.insert("access_token", &token_resp.access_token).unwrap();
    // 4. Chuyển hướng về trang chính
    Ok(Redirect::to("/"))
}
