mod config;
mod di;
mod observability;
mod resilience;
mod routes;
mod features;

use config::{load_settings};
use di::SharedState;
use observability::init_tracing;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    // 1. Init tracing/logging
    init_tracing();

    // 2. Load settings
    let settings = load_settings();

    // 3. Build application state
    let state: SharedState = di::build_state(settings).await?;

    // 4. Build router
    let app = routes::create_router(&state);

    // 5. Start server (Axum 0.8+)
    let addr = format!("0.0.0.0:{}", state.settings.port);
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

