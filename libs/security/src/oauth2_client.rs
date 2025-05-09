// src/security/oauth2_client.rs
use oauth2::{
  basic::BasicClient,
  AuthUrl, ClientId, ClientSecret, TokenUrl,
  RedirectUrl, PkceCodeChallenge, Scope,
};
use url::Url;

pub fn build_epic_oauth_client(
  client_id: &str,
  client_secret: &str,
  authorize_url: &str,
  token_url: &str,
  redirect_uri: &str,
) -> BasicClient {
  // parse URL
  let auth_url = AuthUrl::new(authorize_url.to_string()).expect("invalid authorize URL");
  let token_url = TokenUrl::new(token_url.to_string()).expect("invalid token URL");

  // khởi client
  let mut client = BasicClient::new(
      ClientId::new(client_id.to_string()),
      Some(ClientSecret::new(client_secret.to_string())),
      auth_url,
      Some(token_url),
  );

  // redirect URI
  client = client.set_redirect_uri(
      RedirectUrl::new(redirect_uri.to_string()).expect("invalid redirect URI")
  );

  client
}
