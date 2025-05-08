use serde::Deserialize;
use figment::{Figment, providers::{Env, Yaml}, FigmentError};

/// Cấu hình OAuth2 với Epic
#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2Settings {
    /// Epic OAuth2 client ID
    pub client_id: String,
    /// Epic OAuth2 client secret
    pub client_secret: String,
    /// Epic OAuth2 authorize endpoint
    pub authorize_url: String,
    /// Epic OAuth2 token endpoint
    pub token_url: String,
    /// Redirect URI đã đăng ký trên Epic
    pub redirect_uri: String,
}

/// Cấu hình chung cho API Gateway
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    /// Cổng HTTP server
    pub port: u16,
    /// Khóa bí mật để mã hóa session cookie (hex 32 bytes)
    pub session_key: String,
    /// OAuth2 config
    pub oauth2: OAuth2Settings,
}

impl Settings {
    /// Load cấu hình từ file (nếu có) và biến môi trường
    pub fn load() -> Result<Self, FigmentError> {
        Figment::new()
            // Nếu bạn dùng file config/settings.yaml, bỏ comment dòng sau:
            // .merge(Yaml::file("config/settings.yaml"))
            // Nạp tất cả biến môi trường trực tiếp
            .merge(Env::raw())
            .extract()
    }
}
