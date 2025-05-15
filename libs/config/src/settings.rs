use serde::Deserialize;
use figment::{Figment, providers::{Env, Yaml}, error::Error as FigmentError};

/// Cấu hình OAuth2
#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2Settings {
    /// Epic OAuth2 client ID
    pub client_id: String,
    /// Epic OAuth2 client secret
    pub client_secret: String,
    /// Epic OAuth2 authorize endpoint
    pub token_url: String,
    /// Redirect URI đã đăng ký trên Epic
    pub redirect_uri: String,
    /// auth_url
    pub auth_url: String,
    /// Scopes yêu cầu
    pub scopes: Vec<String>,
    /// Audience yêu cầu
    pub audience: String,
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
