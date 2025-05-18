use std::collections::HashMap;

use serde::Deserialize;
use figment::{Figment, providers::{Env, Yaml}, error::Error as FigmentError};

/// Cấu hình OAuth2
#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2ClientSettings  {
    /// Epic OAuth2 client ID
    pub client_id: String,
    /// Epic OAuth2 client secret
    pub client_secret: Option<String>,
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
    pub private_key_pem: Option<String>, // Đường dẫn đến private key file
    pub private_key_algorithm: Option<String>, // Thuật toán ký cho private key (ví dụ: RS384, ES384)
    pub key_id: Option<String>, // Key ID (kid) để sử dụng trong header JWT và JWKS
}

/// Cấu hình chung cho API Gateway
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    /// Cổng HTTP server
    pub port: u16,
    /// Khóa bí mật để mã hóa session cookie (hex 32 bytes)
    pub session_key: String,
    /// OAuth2 config
    pub oauth_clients: HashMap<String, OAuth2ClientSettings>,
}
