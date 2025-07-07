use crate::di::{AppState, SharedState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_macros::debug_handler;
use oauth2_lib::epic::error::AxumAppError;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}
#[debug_handler]
pub async fn epic_callback_handler(
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
        eprintln!(
            "CSRF token mismatch. Stored: {:?}, Received: {:?}",
            stored_state, query.state
        );
        // Redirect về trang login hoặc trả lỗi rõ ràng hơn
        return Ok(Redirect::to("/auth/error2").into_response());
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
    let app_state = state.as_ref(); // Nếu SharedState là Arc<AppState>
    let epic_client_arc = app_state.oauth_clients.get("epic_sandbox").ok_or_else(|| {
        AxumAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Epic client not found in state".to_string(),
        )
    })?;

    let mut epic_client = epic_client_arc.lock().await;
    // Đổi code lấy access tokenlet csrf_token: String = session.get("csrf_token").await?.ok_or(...)?;
    let csrf_token: String = session.get("csrf_token").await?.ok_or_else(|| {
        AxumAppError::new(
            StatusCode::UNAUTHORIZED,
            "CSRF token not found in session".to_string(),
        )
    })?;
    let pkce_verifier: String = session.get("pkce_verifier").await?.ok_or_else(|| {
        AxumAppError::new(
            StatusCode::UNAUTHORIZED,
            "PKCE verifier not found in session".to_string(),
        )
    })?;
    let session_id = session.id();
    tracing::info!("CALLBACK: session_id={:?}, csrf_token={:?}, pkce_verifier={:?}", session_id, csrf_token, pkce_verifier);
    match epic_client
        .exchange_code(query.code, csrf_token, pkce_verifier, query.state)
        .await
    {
        Ok(token) => {
            // Lưu access token vào session nếu muốn
            if let Err(e) = session.insert("access_token", token.secret()).await {
                eprintln!("Session error while inserting access_token: {e:?}");
                return Err((AxumAppError::from(e)));
            }
            Ok(Redirect::to("/patientsummary").into_response()) // Hoặc trang dashboard
        }
        Err(e) => {
            // Xử lý lỗi, có thể redirect về trang lỗi hoặc login lại
            eprintln!("OAuth2 callback error: {e:?}");
            return Err((AxumAppError::from(e)));
        }
    }
    // return Ok(Redirect::to("/auth/login").into_response());
}
