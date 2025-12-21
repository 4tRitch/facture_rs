pub struct AppCredentials{
  user: String,
  password: String
}

impl AppCredentials {
  pub fn new() -> Self {
    AppCredentials {
      user: String::new(),
      password: String::new()
    }
  }

  pub fn password(mut self, password: impl Into<String>) -> Self{
    self.password = password.into();
    self
  }

  pub fn user(mut self, user: impl Into<String>) -> Self{
    self.user = user.into();
    self
  }

  pub fn get_password(&mut self) -> String { self.password.clone() }
  pub fn get_user(&mut self) -> String { self.user.clone() }

}
