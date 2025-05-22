// /workspaces/seds_server/services/api-gateway/src/routes/jwks.rs (File mới)
use crate::di::SharedState; // Hoặc AppState nếu bạn dùng trực tiếp
use axum::{extract::State, response::Json, routing::get, Router};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use config_lib::settings::OAuth2ClientSettings; // Import struct cấu hình client
use jsonwebtoken::jwk; // For Jwk type
use rsa::{
    pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey, traits::PublicKeyParts, RsaPrivateKey,
    RsaPublicKey,
};
use serde_json::json;
use std::sync::Arc; // For Base64URL encoding

fn create_jwk_from_settings(client_settings: &OAuth2ClientSettings) -> Option<serde_json::Value> {
    // Ensure all required fields for JWK generation from private key are present
    if client_settings.private_key_pem.is_none()
        || client_settings.key_id.is_none()
        || client_settings.private_key_algorithm.is_none()
        || client_settings.private_key_pem.as_ref().unwrap().trim().is_empty() // Also check if PEM is empty
    {
        return None;
    }

    let key_id = client_settings.key_id.as_ref().unwrap();
    let algorithm_str = client_settings.private_key_algorithm.as_ref().unwrap();
    let private_key_pem = client_settings.private_key_pem.as_ref().unwrap();

    // Step 1: Parse the private key PEM to get public components (n, e)
    // Try parsing as PKCS#8 first, then PKCS#1 as a fallback.
    // `openssl genrsa` typically produces PKCS#1 by default.
    let rsa_public_key_result = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map(|priv_key| RsaPublicKey::from(&priv_key))
        .or_else(|pkcs8_parse_err| {
            tracing::warn!("Failed to parse PEM as PKCS#8 for KID {}: {}. Trying PKCS#1 as fallback.", key_id, pkcs8_parse_err);
            RsaPrivateKey::from_pkcs1_pem(private_key_pem)
                .map(|priv_key| RsaPublicKey::from(&priv_key))
                .map_err(|pkcs1_parse_err| {
                    // If both attempts fail, log both errors. The pkcs8_parse_err is likely more relevant for "BEGIN PRIVATE KEY".
                    tracing::error!("Failed to parse PEM as PKCS#8 or PKCS#1 for KID {}. PKCS#8 Error: {}, PKCS#1 Error: {}", key_id, pkcs8_parse_err, pkcs1_parse_err);
                    pkcs8_parse_err // Return the error from the primary attempt (PKCS#8)
                })
        });

    match rsa_public_key_result {
        Ok(public_key) => {
            // Step 2: Extract n and e, then Base64URL encode them
            let n_bytes = public_key.n().to_bytes_be();
            let e_bytes = public_key.e().to_bytes_be();

            let n_b64u = URL_SAFE_NO_PAD.encode(n_bytes);
            let e_b64u = URL_SAFE_NO_PAD.encode(e_bytes);

            // Step 3: Use the algorithm string from config directly for the 'alg' field.
            // Assuming the configured algorithm is one of the supported RSA algorithms by Epic (RS256, RS384, RS512).
            // If EC keys were supported, this logic would need to be expanded.
            let alg_field = algorithm_str.to_uppercase();

            // Step 4: Construct the JWK JSON using serde_json::json! macro
            let jwk_json = json!({
                "kty": "RSA", // Assuming RSA based on algorithm_str check or key type
                "n": n_b64u,
                "e": e_b64u,
                "kid": key_id,
                "alg": alg_field,
                "use": "sig" // "sig" for signature
            });

            // Optional: Validate by deserializing into Jwk struct, then re-serialize
            // This ensures the structure is valid according to jsonwebtoken::jwk::Jwk
            match serde_json::from_value::<jwk::Jwk>(jwk_json.clone()) {
                Ok(parsed_jwk) => Some(serde_json::to_value(parsed_jwk).unwrap_or(jwk_json)),
                Err(e) => {
                    tracing::error!(
                        "Failed to validate constructed JWK JSON for KID {}: {}. Raw JSON: {}",
                        key_id,
                        e,
                        jwk_json
                    );
                    None
                }
            }
        }
        Err(e) => {
            tracing::error!(
                "Failed to parse RSA private key PEM for KID {}: {}",
                key_id,
                e
            );
            None
        }
    }
}

async fn jwks_handler(State(state): State<SharedState>) -> impl axum::response::IntoResponse {
    let app_state = state.as_ref();
    let mut keys = Vec::new();

    for (_client_name, client_settings) in &app_state.settings.oauth_clients {
        // Chỉ tạo JWK cho các client có cấu hình private key và key_id
        if client_settings.private_key_pem.is_some()
            && client_settings.key_id.is_some()
            && client_settings.private_key_algorithm.is_some()
            && !client_settings.private_key_pem.as_ref().unwrap().trim().is_empty()
        {
            tracing::debug!("Attempting to create JWK for client: {}", _client_name);
            if let Some(jwk) = create_jwk_from_settings(client_settings) {
                keys.push(jwk);
            }
        }
    }
    Json(json!({ "keys": keys }))
}
// Modified jwks_handler to return impl IntoResponse for adding headers
async fn jwks_handler_with_headers(State(state): State<SharedState>) -> impl axum::response::IntoResponse {
    let json_response = jwks_handler(State(state)).await;
    // Add caching headers as recommended by Epic
    (
        [(axum::http::header::CACHE_CONTROL, "public, max-age=2592000")], // Cache for 30 days (adjust as needed)
        json_response,
    )
}
pub fn jwks_routes(state: &SharedState) -> Router {
    Router::new()
        .route("/.well-known/jwks.json", get(jwks_handler_with_headers))
        .with_state(state.clone())
}
