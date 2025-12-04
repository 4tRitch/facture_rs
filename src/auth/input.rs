use crate::auth::{app::App, credentials::AuthCredentials};

pub struct PasswordInput {
  pub app: App,
  pub credentials: AuthCredentials,
  pub scopes: String,
}

impl PasswordInput {
  pub fn new() -> Self{
    PasswordInput{
      app:App::new(),
      credentials: AuthCredentials::new(),
      scopes: String::new()
    }
  }
}


pub struct RefreshInput {
  pub app: App,
  pub refresh_token: String,
}

impl RefreshInput {
  pub fn new() -> Self{
    RefreshInput{
      app:App::new(),
      refresh_token: String::new()
    }
  }
}
