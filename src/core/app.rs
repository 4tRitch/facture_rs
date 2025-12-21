use crate::core::credentials::AppCredentials;

pub struct App{
  secret: String,
  id: String,
  pub credentials: AppCredentials
}

impl App {
  pub fn new() -> Self {
    App {
      secret:String::new(),
      id:String::new(),
      credentials: AppCredentials::new()
    }
  }


  pub fn set_client_id(mut self, client_id: impl Into<String>) -> Self{
    self.id = client_id.into();
    self
  }

  pub fn set_client_secret(mut self, secret: impl Into<String>) -> Self{
    self.secret = secret.into();
    self
  }

  pub fn set_credentials(mut self, credentials: impl Into<AppCredentials>) -> Self{
    self.credentials = credentials.into();
    self
  }


  pub fn get_client_id(&mut self) -> String { self.id.clone() }
  pub fn get_client_secret(&mut self) -> String { self.secret.clone() }

}

