use crate::di::{AxumAppError, SharedState}; // Adjust path if AppError is elsewhere
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

use super::callback::epic_callback_handler;
use super::login::epic_login_handler; // Adjust path if handlers are elsewhere

// If AuthCallbackParams is in auth/mod.rs or auth.rs, you might need:
// use super::AuthCallbackParams;

// ... (AuthCallbackParams struct definition, or import it) ...

pub fn auth_routes(state: &SharedState) -> Router {
    // Make sure this function is public
    Router::new()
        .route("/epic-sandbox/login", get(epic_login_handler)) // Assuming handlers are in auth/handlers.rs
        .route("/epic-sandbox/callback", get(epic_callback_handler))
        // ... other auth routes ...
        .with_state(state.clone())
}
