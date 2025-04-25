#[derive(Deserialize)]
pub struct OAuth2Settings {
  pub client_id: String,
  pub client_secret: String,
  pub authorize_url: String,
  pub token_url: String,
  pub redirect_uri: String,
}

#[derive(Deserialize)]
pub struct Settings {
  // ... các trường cũ …
  pub oauth2: OAuth2Settings,
}
