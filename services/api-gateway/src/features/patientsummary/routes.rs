use axum::{routing::get, Router};

use crate::{di::SharedState, features::patientsummary::handlers::{get_demo_patients, get_demo_summary, patient_summary_handler}};

pub fn patient_summary_routes(state: &SharedState) -> Router {
    Router::new()
        .route("/api/patient/{id}/summary", get(patient_summary_handler))
        .route("/demo/patients", get(get_demo_patients))
        .route("/demo/patients/{id}/summary", get(get_demo_summary))
        .with_state(state.clone())
}
