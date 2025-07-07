use axum::{routing::get, Router};

use crate::{di::SharedState, features::patientsummary::handlers::patient_summary_handler};

pub fn patient_summary_routes(state: &SharedState) -> Router {
    Router::new()
        .route("/api/patient/{id}/summary", get(patient_summary_handler))
        .with_state(state.clone())
}
