use axum::{body::{Body, to_bytes}, extract::Request, http::{HeaderMap, Uri}, response::IntoResponse};
use reqwest::{Client, StatusCode};

pub async fn proxy_fresh(uri: Uri, req: Request<Body>) -> impl IntoResponse {
    let client = Client::new();
    let path = uri.path();
    let full_uri = format!("http://localhost:8000{}", path);
    let (parts, body) = req.into_parts();
    let max_body_size = 1024 * 1024; // 1 MB
    let body_bytes = match to_bytes(body, max_body_size).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read request body: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                "Failed to read incoming request body",
            )
                .into_response();
        }
    };

    // Build the request to Fresh
    let mut fresh_req = client
        .request(parts.method.clone(), &full_uri)
        .body(body_bytes.clone()); // forward body

    for (key, value) in parts.headers.iter() {
        fresh_req = fresh_req.header(key, value);
    }

    match fresh_req.send().await {
        Ok(resp) => {
            let status = resp.status();
            // Clone all headers (Content-Type, etc.)
            let mut axum_headers = HeaderMap::new();
            for (k, v) in resp.headers() {
                axum_headers.insert(k.clone(), v.clone());
            }

            let body = match resp.bytes().await {
                Ok(body) => body,
                Err(e) => {
                    println!("Failed to read proxy body: {}", e);
                    return (
                        StatusCode::BAD_GATEWAY,
                        "Failed to read body from frontend".to_string(),
                    )
                        .into_response();
                }
            };

            (status, axum_headers, body).into_response()
        }
        Err(err) => {
            println!("Proxy to Fresh failed: {}", err);
            (
                StatusCode::BAD_GATEWAY,
                "Failed to connect to frontend".to_string(),
            )
                .into_response()
        }
    }
}