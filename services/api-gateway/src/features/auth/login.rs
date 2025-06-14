use std::sync::Arc;

use crate::di::{AppState, SharedState};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use oauth2_lib::epic::{client::EpicFhirClient, error::AxumAppError};
use reqwest::StatusCode;
use tower_sessions::{MemoryStore, Session};

/// Route /auth/login sử dụng tower_sessions::Session
pub async fn epic_login_handler(
    State(state): State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse, AxumAppError> {
    let app_state = state.as_ref(); // Nếu SharedState là Arc<AppState>
    let epic_client_arc = app_state.oauth_clients.get("epic_sandbox").ok_or_else(|| {
        AxumAppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Epic client not found in state".to_string(),
        )
    })?;

    let mut epic_client = epic_client_arc.lock().await; // Lấy MutexGuard

    let (auth_url, csrf_token, pkce_verifier) = epic_client.get_authorization_url()?;
    session.insert("pkce_verifier", &pkce_verifier).await?;
    session
        .insert("csrf_token", &csrf_token)
        .await
        .map_err(AxumAppError::from)?;
    let session_id = session.id();
    tracing::info!("LOGIN: session_id={:?}, csrf_token={:?}, pkce_verifier={:?}", session_id, csrf_token, pkce_verifier);

    Ok(Redirect::to(auth_url.as_ref()))
}
