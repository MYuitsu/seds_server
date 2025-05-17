use std::sync::Arc;
use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::{Redirect, IntoResponse},
};
use axum_macros::debug_handler;
use oauth2_lib::epic::error::AxumAppError;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_sessions::Session;
use crate::di::{AppState, SharedState};

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}
#[debug_handler]
pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CallbackQuery>,
    session: Session,
) -> Result<impl IntoResponse, AxumAppError> {
    // Lấy lại CSRF token từ session
    let stored_state: Option<String> = match session.get("csrf_token").await {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Session error while getting csrf_token: {e:?}");
            // Trả về lỗi hoặc redirect đến trang login/error
            return Err((AxumAppError::from(e)));
        }
    };

    if stored_state.as_deref() != Some(&query.state) {
        eprintln!("CSRF token mismatch. Stored: {:?}, Received: {:?}", stored_state, query.state);
        // Redirect về trang login hoặc trả lỗi rõ ràng hơn
        return Ok(Redirect::to("/auth/login").into_response());
    }

    // Lấy mutable EpicFhirClient từ state
    // let mut epic_client = match state.epic_client.lock().await {
    //     Ok(client) => client,
    //     Err(poison_error) => {
    //         // When the lock is poisoned, don't try to convert the PoisonError directly
    //         // as it contains the non-Send MutexGuard.
    //         // Instead, create a specific error for this situation.
    //         eprintln!("Failed to lock EpicFhirClient due to mutex poisoning: {:?}", poison_error);
    //         // Assuming AxumAppError has a constructor like `new(StatusCode, String)`
    //         // Try using From<(StatusCode, String)>
    //             return Err(AxumAppError::new(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Internal server error: Could not acquire resource lock.".to_string()
    //         ))
    //      }
    // };
    let mut epic_client = state.epic_client.lock().await;

    // Đổi code lấy access token
    match epic_client.exchange_code_for_token(query.code, query.state).await {
        Ok(token) => {
            // Lưu access token vào session nếu muốn
            if let Err(e) = session.insert("access_token", token.secret()).await {
                eprintln!("Session error while inserting access_token: {e:?}");
                return Err((AxumAppError::from(e)));
            }
            Ok(Redirect::to("/").into_response()) // Hoặc trang dashboard
        }
        Err(e) => {
            // Xử lý lỗi, có thể redirect về trang lỗi hoặc login lại
            eprintln!("OAuth2 callback error: {e:?}");
            return Err((AxumAppError::from(e)));
        }
    }
    // return Ok(Redirect::to("/auth/login").into_response());
}
