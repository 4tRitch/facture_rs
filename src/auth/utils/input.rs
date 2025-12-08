use crate::auth::utils::{app::App, credentials::AuthCredentials};


// Password
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


//Refresh Token
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

// Authorize
pub struct AuthorizeInput {
  pub app: App,
  pub scopes: String,
  pub code: String,
  pub redirect_uri: String
}

impl AuthorizeInput {
  pub fn new() -> Self{
    AuthorizeInput{
      app:App::new(),
      scopes: String::new(),
      code: String::new(),
      redirect_uri: String::new()
    }
  }
}

