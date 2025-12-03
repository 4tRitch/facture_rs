use std::io;

pub enum PLATFORM {
   FactureApp,
   MiContador,
   None
}

pub struct Client{
  pub platform: PLATFORM,
  pub secret: String,
  pub id: String
}

impl Client {
  pub fn new() -> Self {
    Client {
      platform: PLATFORM::None,
      secret:String::new(),
      id:String::new()
    }
  }
}

pub struct Auth{
  access_token: String,
  refresh_token: String
}

pub struct AuthResponse{
  access_token: String,
  refresh_token: String,
  expires_in: usize
}
impl AuthResponse {
  pub fn new() -> Self {
    AuthResponse {
      access_token: String::new(),
      refresh_token: String::new(),
      expires_in: 0
    }
  }
}

impl Auth {
  pub fn new() -> Self {
    Auth {
      access_token: String::new(),
      refresh_token: String::new()
    }
  }

  pub fn password(&mut self, client: Client) -> Result<(),io::Error>{
    let mut response = AuthResponse::new();
    let _ = client.platform;

    response.access_token = "".to_string();


    self.access_token = response.access_token;
    self.refresh_token = response.refresh_token;
    Ok(())
  }

  pub fn get_access_token(&mut self) -> String { self.access_token.clone() }
  pub fn get_refresh_token(&mut self) -> String { self.refresh_token.clone() }

}
