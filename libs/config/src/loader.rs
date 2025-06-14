use crate::settings::Settings;
use figment::error::Error as FigmentError;
use figment::{
    Figment,
    providers::{Env, Format, Yaml},
};

/// Load application settings in this order:
/// 1. config/default.yaml
/// 2. config/{APP_ENV}.yaml (where APP_ENV defaults to "development")
/// 3. environment variables prefixed with "APP_"
///
/// # Panics
/// Panics if it cannot read or deserialize the final Settings.
pub fn load() -> Settings {
    // 1. Determine the runtime environment (default = "development")
    let current_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    // 2. Build Figment object, merging providers in precedence order
    let figment = Figment::new()
        // Base config
        .merge(Yaml::file("config/default.yaml"))
        // Env-specific override (e.g. config/production.yaml)
        .merge(Yaml::file(&format!("config/{}.yaml", current_env)))
        // Override with environment variables.
        // Figment sẽ cố gắng map APP_OAUTH_CLIENTS_EPIC_SANDBOX_CLIENT_ID thành settings.oauth_clients.epic_sandbox.client_id
        .merge(Env::prefixed("APP_").split("_")); // Giữ prefix chung, figment sẽ xử lý lồng nhau

    // 3. Extract into our Settings struct, returning a Result
    figment
        .extract::<Settings>()
        .map_err(|e: FigmentError| {
            eprintln!("\nFATAL: Could not load application configuration.");
            eprintln!("Error details: {:#?}", e);
            eprintln!("Please ensure 'config/default.yaml' exists and is correctly formatted.");
            eprintln!("If using environment-specific config (e.g., 'config/{}.yaml'), check it as well.", current_env);
            eprintln!("Also, verify that all required fields (like 'port', 'host', 'oauth2.client_id', etc.) are present in your configuration sources or environment variables (prefixed with APP_).\n");
            e
        })
        .expect("Critical error loading application configuration. See details above.")
}
