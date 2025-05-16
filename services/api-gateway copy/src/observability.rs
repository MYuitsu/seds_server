use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Khởi tạo tracing/logging với env filter
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt::fmt()
        .with_env_filter(filter)
        .init();
}
