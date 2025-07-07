use axum::{body::Body, extract::Request, http::{HeaderMap, Uri}, response::{IntoResponse, Response}};
use reqwest::{Client, StatusCode};

pub async fn proxy_fresh(uri: Uri, req: Request<Body>) -> impl IntoResponse {
    let client = Client::new();
    let path = uri.path();
    let full_uri = format!("http://localhost:8000{}", path);
    let mut fresh_req = client.request(req.method().clone(), &full_uri);

    for (key, value) in req.headers() {
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