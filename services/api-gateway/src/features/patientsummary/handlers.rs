use anyhow::Ok;
use axum::{extract::Path, http::{HeaderMap, HeaderValue}, response::IntoResponse};
use oauth2_lib::epic::error::AxumAppError;
use reqwest::{header, Client, StatusCode};
use tower_sessions::Session;

pub async fn patient_summary_handler(
    session: Session,
    Path(patient_id): Path<String>,
) -> Result<impl IntoResponse, AxumAppError> {
    // Extract the access token from the session
    // let token = String::from("demo-token");
    let token = session.get::<String>("access_token").await
        .map_err(|err| {
            tracing::error!("Failed to retrieve access token from session: {}", err);
            AxumAppError::new(
                StatusCode::UNAUTHORIZED,
                format!("Failed to retrieve access token from session: {}", err),
            )
        })?
        .ok_or_else(|| {
            tracing::error!("Access token not found in session");
            AxumAppError::new(
                StatusCode::UNAUTHORIZED,
                "Access token not found in session".to_string(),
            )
        })?;

    // Create the HTTP client and construct the URL
    let client = Client::new();
    let url = format!("http://0.0.0.0:3010/patient_summary/{}", patient_id);

    // Send the request to the downstream service
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Failed to contact downstream service: {}", err);
            AxumAppError::new(
                StatusCode::BAD_GATEWAY,
                format!("Failed to contact downstream service: {}", err),
            )
        })?;

    // Handle the response from the downstream service
    match resp.status() {
        StatusCode::OK => {
            let content_type = resp
                .headers()
                .get(header::CONTENT_TYPE)
                .cloned()
                .unwrap_or_else(|| HeaderValue::from_static("application/json"));

            let body = resp.bytes().await.map_err(|err| {
                tracing::error!("Failed to read response body: {}", err);
                AxumAppError::new(
                    StatusCode::BAD_GATEWAY,
                    format!("Failed to read response body: {}", err),
                )
            })?;

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type);

            Ok((headers, body).into_response()).map_err(|err| {
                AxumAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to construct response: {}", err),
                )
            })
        }
        status => {
            let error_message = format!(
                "Downstream service returned error: {} for patient ID: {}",
                status, patient_id
            );
            tracing::warn!("{}", error_message);
            Err(AxumAppError::new(status, error_message))
        }
    }
}

pub async fn get_demo_patients() -> Result<impl IntoResponse, AxumAppError> {
    // Create the HTTP client and construct the URL
    let client = Client::new();
    let url = format!("http://0.0.0.0:3010/demo/patients");
    // Send the request to the downstream service
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Failed to contact downstream service: {}", err);
            AxumAppError::new(
                StatusCode::BAD_GATEWAY,
                format!("Failed to contact downstream service: {}", err),
            )
        })?;
    // Handle the response from the downstream service
    match resp.status() {
        StatusCode::OK => {
            let content_type = resp
                .headers()
                .get(header::CONTENT_TYPE)
                .cloned()
                .unwrap_or_else(|| HeaderValue::from_static("application/json"));

            let body = resp.bytes().await.map_err(|err| {
                tracing::error!("Failed to read response body: {}", err);
                AxumAppError::new(
                    StatusCode::BAD_GATEWAY,
                    format!("Failed to read response body: {}", err),
                )
            })?;

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type);

            Ok((headers, body).into_response()).map_err(|err| {
                AxumAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to construct response: {}", err),
                )
            })
        }
        status => {
            let error_message = format!("Downstream service returned error.");
            tracing::warn!("{}", error_message);
            Err(AxumAppError::new(status, error_message))
        }
    }
}

pub async fn get_demo_summary(Path(patient_id): Path<String>) -> Result<impl IntoResponse, AxumAppError> {
    // Create the HTTP client and construct the URL
    let client = Client::new();
    let url = format!("http://0.0.0.0:3010/demo/patients/{}/summary", patient_id);
    // Send the request to the downstream service
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Failed to contact downstream service: {}", err);
            AxumAppError::new(
                StatusCode::BAD_GATEWAY,
                format!("Failed to contact downstream service: {}", err),
            )
        })?;
    // Handle the response from the downstream service
    match resp.status() {
        StatusCode::OK => {
            let content_type = resp
                .headers()
                .get(header::CONTENT_TYPE)
                .cloned()
                .unwrap_or_else(|| HeaderValue::from_static("application/json"));

            let body = resp.bytes().await.map_err(|err| {
                tracing::error!("Failed to read response body: {}", err);
                AxumAppError::new(
                    StatusCode::BAD_GATEWAY,
                    format!("Failed to read response body: {}", err),
                )
            })?;

            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type);

            Ok((headers, body).into_response()).map_err(|err| {
                AxumAppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to construct response: {}", err),
                )
            })
        }
        status => {
            let error_message = format!("Downstream service returned error.");
            tracing::warn!("{}", error_message);
            Err(AxumAppError::new(status, error_message))
        }
    }
}
