// /workspaces/seds_server/services/api-gateway/src/routes/jwks.rs (File mới)
use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::json;
use std::sync::Arc;
use crate::di::SharedState; // Hoặc AppState nếu bạn dùng trực tiếp
use config_lib::settings::OAuth2ClientSettings; // Import struct cấu hình client
use jsonwebtoken::jwk; // For Jwk type
use rsa::{pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey, traits::PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _}; // For Base64URL encoding

fn create_jwk_from_settings(client_settings: &OAuth2ClientSettings) -> Option<serde_json::Value> {
    if client_settings.private_key_pem.is_none() || client_settings.key_id.is_none() || client_settings.private_key_algorithm.is_none() {
        return None;
    }

    let key_id = client_settings.key_id.as_ref().unwrap();
    let algorithm_str = client_settings.private_key_algorithm.as_ref().unwrap();
    let private_key_pem = client_settings.private_key_pem.as_ref().unwrap();

    // Step 1: Parse the private key PEM to get public components (n, e)
    // Try parsing as PKCS#8 first, then PKCS#1 as a fallback.
    // `openssl genrsa` typically produces PKCS#1 by default.
    let rsa_public_key_result = RsaPrivateKey::from_pkcs1_pem(private_key_pem)
        .map(|priv_key| RsaPublicKey::from(&priv_key))
        .or_else(|pkcs8_err| {
            tracing::debug!("Failed to parse PEM as PKCS#8 for KID {}: {}. Trying PKCS#1.", key_id, pkcs8_err);
            RsaPrivateKey::from_pkcs1_pem(private_key_pem)
                .map(|priv_key| RsaPublicKey::from(&priv_key))
                .map_err(|pkcs1_err| {
                    tracing::error!("Failed to parse PEM as PKCS#8 or PKCS#1 for KID {}: PKCS#8 Error: {}, PKCS#1 Error: {}", key_id, pkcs8_err, pkcs1_err);
                    pkcs1_err // Return the last error
                })
        });

    match rsa_public_key_result {
        Ok(public_key) => {
            // Step 2: Extract n and e, then Base64URL encode them
            let n_bytes = public_key.n().to_bytes_be();
            let e_bytes = public_key.e().to_bytes_be();

            let n_b64u = URL_SAFE_NO_PAD.encode(n_bytes);
            let e_b64u = URL_SAFE_NO_PAD.encode(e_bytes);

            // Step 3: Map the algorithm string from config to jsonwebtoken::jwk::Algorithm
            // This is for the 'alg' field in the JWK.
            let jwk_alg_enum = match algorithm_str.to_uppercase().as_str() {
                "RS256" => jsonwebtoken::Algorithm::RS256,
                "RS384" => jsonwebtoken::Algorithm::RS384,
                "RS512" => jsonwebtoken::Algorithm::RS512,
                "ES256" => jsonwebtoken::Algorithm::ES256,
                "ES384" => jsonwebtoken::Algorithm::ES384,
                "HS256" => jsonwebtoken::Algorithm::HS256,
                "HS384" => jsonwebtoken::Algorithm::HS384,
                "HS512" => jsonwebtoken::Algorithm::HS512,
                "PS256" => jsonwebtoken::Algorithm::PS256,
                "PS384" => jsonwebtoken::Algorithm::PS384,
                "PS512" => jsonwebtoken::Algorithm::PS512,
                "EDDSA" => jsonwebtoken::Algorithm::EdDSA,
                _ => {
                    tracing::warn!("Unsupported algorithm string for JWK: {} for KID {}", algorithm_str, key_id);
                    return None;
                }
            };

            // Step 4: Construct the JWK JSON using serde_json::json! macro
            let jwk_json = json!({
                "kty": "RSA", // Assuming RSA based on algorithm_str check or key type
                "n": n_b64u,
                "e": e_b64u,
                "kid": key_id,
                "alg": match jwk_alg_enum {
                    jsonwebtoken::Algorithm::RS256 => "RS256",
                    jsonwebtoken::Algorithm::RS384 => "RS384",
                    jsonwebtoken::Algorithm::RS512 => "RS512",
                    jsonwebtoken::Algorithm::ES256 => "ES256",
                    jsonwebtoken::Algorithm::ES384 => "ES384",
                    jsonwebtoken::Algorithm::HS256 => "HS256",
                    jsonwebtoken::Algorithm::HS384 => "HS384",
                    jsonwebtoken::Algorithm::HS512 => "HS512",
                    jsonwebtoken::Algorithm::PS256 => "PS256",
                    jsonwebtoken::Algorithm::PS384 => "PS384",
                    jsonwebtoken::Algorithm::PS512 => "PS512",
                    jsonwebtoken::Algorithm::EdDSA => "EdDSA",
                },
                "use": "sig" // "sig" for signature
            });

            // Optional: Validate by deserializing into Jwk struct, then re-serialize
            // This ensures the structure is valid according to jsonwebtoken::jwk::Jwk
            match serde_json::from_value::<jwk::Jwk>(jwk_json.clone()) {
                Ok(parsed_jwk) => Some(serde_json::to_value(parsed_jwk).unwrap_or(jwk_json)),
                Err(e) => {
                    tracing::error!("Failed to validate constructed JWK JSON for KID {}: {}. Raw JSON: {}", key_id, e, jwk_json);
                    None
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to parse RSA private key PEM for KID {}: {}", key_id, e);
            None
        }
    }
}


async fn jwks_handler(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let app_state = state.as_ref();
    let mut keys = Vec::new();

    for (_client_name, client_settings) in &app_state.settings.oauth_clients {
        // Chỉ tạo JWK cho các client có cấu hình private key và key_id
        if client_settings.private_key_pem.is_some() && client_settings.key_id.is_some() {
            if let Some(jwk) = create_jwk_from_settings(client_settings) {
                keys.push(jwk);
            }
        }
    }
    Json(json!({ "keys": keys }))
}

pub fn jwks_routes(state: &SharedState) -> Router {
    Router::new()
        .route("/.well-known/jwks.json", get(jwks_handler))
        .with_state(state.clone())
}
