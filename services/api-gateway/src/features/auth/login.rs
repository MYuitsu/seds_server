use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Redirect}};
use oauth2_lib::epic::{client::EpicFhirClient, error::AxumAppError};
use tower_sessions::{MemoryStore, Session};
use crate::di::{AppState, SharedState};

/// Route /auth/login sử dụng tower_sessions::Session
pub async fn login(
    State(state): State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse, AxumAppError> {

    let (auth_url, csrf_token) = state.epic_client
        .lock().await
        .get_authorization_url()
        .map_err(|e| AxumAppError::from(e))?;


    session
        .insert("csrf_token", &csrf_token)
        .await
        .map_err(AxumAppError::from)?;

    Ok(Redirect::to(auth_url.as_ref()))
}