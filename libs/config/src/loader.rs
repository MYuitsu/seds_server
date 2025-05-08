use figment::{
    Figment, 
    providers::{Env, Yaml}
};
use crate::settings::Settings;

/// Load application settings in this order:
/// 1. config/default.yaml
/// 2. config/{APP_ENV}.yaml (where APP_ENV defaults to "development")
/// 3. environment variables prefixed with "APP_"
///
/// # Panics
/// Panics if it cannot read or deserialize the final Settings.
pub fn load() -> Settings {
    // 1. Determine the runtime environment (default = "development")
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    // 2. Build Figment object, merging providers in precedence order
    let figment = Figment::new()
        // Base config
        .merge(Yaml::file("config/default.yaml"))
        // Env-specific override (e.g. config/production.yaml)
        .merge(Yaml::file(&format!("config/{}.yaml", env)).nested())
        // Finally, override with any APP_* environment variable
        .merge(Env::prefixed("APP_").split("_"));

    // 3. Extract into our Settings struct
    figment
        .extract::<Settings>()
        .unwrap_or_else(|e| panic!("Failed to load configuration: {}", e))
}
