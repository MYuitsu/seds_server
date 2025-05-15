use axum::{
    extract::{State, Query},
    response::{Redirect, IntoResponse},
};
use serde::Deserialize;
use tower_sessions::Session;
use crate::di::SharedState;

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

pub async fn callback(
    State(state): State<SharedState>,
    Query(query): Query<CallbackQuery>,
    session: Session,
) -> impl IntoResponse {
    // Lấy lại CSRF từ session
    let stored_state: Option<String> = session.get("oauth_state").await.unwrap();
    if stored_state.as_deref() != Some(&query.state) {
        return Redirect::to("/auth/login"); // Hoặc trả về lỗi xác thực
    }

    // Lấy mutable EpicFhirClient từ state
    let mut epic_client = state.epic_client.lock().unwrap();

    // Đổi code lấy access token
    match epic_client.exchange_code_for_token(query.code, query.state).await {
        Ok(token) => {
            // Lưu access token vào session nếu muốn
            session.insert("access_token", token.secret()).await.unwrap();
            Redirect::to("/") // Hoặc trang dashboard
        }
        Err(e) => {
            // Xử lý lỗi, có thể redirect về trang lỗi hoặc login lại
            eprintln!("OAuth2 callback error: {e:?}");
            Redirect::to("/auth/login")
        }
    }
}
