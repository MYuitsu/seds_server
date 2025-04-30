use axum::{extract::State, response::Redirect};
use uuid::Uuid;
use crate::{AppState};

pub async fn login(State(state): State<AppState>) -> Redirect {
    let state_token = Uuid::new_v4().to_string();
    // lưu state_token vào session (axum-sessions) để verify callback
    state.session.insert("oauth_state", &state_token).unwrap();
    let url = state.oauth.authorize_url(&state_token);
    Redirect::temporary(&url)
}
