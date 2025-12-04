use serde::{Deserialize, Serialize, de};
use thiserror::Error;

use crate::cons;

#[derive(Debug, Error)]
pub enum AuthError {
  #[error("network error: {0}")]
  Network(#[from] reqwest::Error),

  #[error("parse error: {0}")]
  Parse(#[from] serde_json::Error),

  #[error("api error {status}: {message}")]
  Api {
    status: u16,
    message: String,
  },
}




pub struct AuthClient{
  secret: String,
  id: String,
  credentials: Credentials
}

impl AuthClient {
  pub fn new() -> Self {
    AuthClient {
      secret:String::new(),
      id:String::new(),
      credentials: Credentials::new()
    }
  }

  pub fn password(mut self, password: impl Into<String>) -> Self{
    self.credentials.password = password.into();
    self
  }

  pub fn user(mut self, user: impl Into<String>) -> Self{
    self.credentials.user = user.into();
    self
  }

  pub fn client_id(mut self, client_id: impl Into<String>) -> Self{
    self.id = client_id.into();
    self
  }

  pub fn secret(mut self, secret: impl Into<String>) -> Self{
    self.secret = secret.into();
    self
  }


  pub fn get_password(&mut self) -> String { self.credentials.password.clone() }
  pub fn get_user(&mut self) -> String { self.credentials.user.clone() }
  pub fn get_client_id(&mut self) -> String { self.id.clone() }
  pub fn get_secret(&mut self) -> String { self.secret.clone() }

}


pub  struct Credentials{
  user: String,
  password: String
}

impl Credentials {
  pub fn new() -> Self {
    Credentials {
      user: String::new(),
      password: String::new()
    }
  }
}


#[derive(Serialize,Deserialize, Debug)]
pub struct AuthResponse{
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in: usize
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



#[derive(Debug)]
pub struct Password;

impl Password {
  pub fn new() -> Self { Password {} }

  pub async fn password(&mut self, mut client: AuthClient, scopes: String) -> Result<AuthResponse, AuthError>{
    let req_client = reqwest::Client::new();
    let uri = format!("{}/authorize", cons::URI);

    let params = [
      ("username", &client.get_user()),
      ("password", &client.get_password()),
      ("scope", &scopes),
      ("client_id", &client.get_client_id()),
      ("client_secret", &client.get_secret())
    ];


    let api_response = req_client
      .post(uri)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .query(&[("grant_type", "password")])
      .form(&params)
      .send()
      .await?;

    if !api_response.status().is_success(){
      return Err(AuthError::Api { status: api_response.status().as_u16() , message: api_response.text().await.unwrap() })
    }
    else{
      let json = api_response.text().await?;
      let response: AuthResponse = serde_json::from_str(json.as_str()).unwrap();
      return Ok(response);
    }

  }


}
