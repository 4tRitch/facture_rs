pub struct App{
  secret: String,
  id: String,
}

impl App {
  pub fn new() -> Self {
    App {
      secret:String::new(),
      id:String::new()
    }
  }


  pub fn client_id(mut self, client_id: impl Into<String>) -> Self{
    self.id = client_id.into();
    self
  }

  pub fn secret(mut self, secret: impl Into<String>) -> Self{
    self.secret = secret.into();
    self
  }


  pub fn get_client_id(&mut self) -> String { self.id.clone() }
  pub fn get_secret(&mut self) -> String { self.secret.clone() }

}

