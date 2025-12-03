use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
  #[error("client connection failed: {0}")]
  ConnectionFailed(String),

  #[error("invalid credentials")]
  InvalidCredentials,

  #[error("unexpected error: {0}")]
  Unexpected(String),
}


pub struct Client{
  pub secret: String,
  pub id: String,
  credentials: Credentials
}

impl Client {
  pub fn new() -> Self {
    Client {
      secret:String::new(),
      id:String::new(),
      credentials: Credentials::new()
    }
  }

  pub fn set_credentials(&mut self, user: String, password: String){
    self.credentials.user = user;
    self.credentials.password = password;
  }

  pub fn get_password(&mut self) -> String { self.credentials.password.clone() }
  pub fn get_user(&mut self) -> String { self.credentials.user.clone() }

}

pub struct Auth{
  access_token: String,
  refresh_token: String
}

struct Credentials{
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

  pub fn password(&mut self, client: Client) -> Result<AuthResponse, AuthError>{
    let mut response = AuthResponse::new();

    // // let params = RequestParams{
    //  // grant_type: "".to_string(),
    // //  redirect_uri: "".to_string(),
    // //  code: "".to_string(),
    // //  scope: "".to_string(),
    // // };
    //
    // let params = RequestParams{
    //  grant_type: "".to_string(),
    // };
    //
    // let params = RequestHeaders{
    //  Content_Type: "".to_string(),
    // };
    //
    // let x_www_form_urlencoded = X_WWW_FORM_URLENCODED{
    //  username: client.get_user(),
    //  password: client.get_password(),
    //  scope: "".to_string(),
    //  client_id: client.id,
    //  client_secret: client.secret,
    // }
    //
    //
    // let request = FactureRequest::new();
    // request.set_params(params);
    // request.set_headers(headers);
    // if /* !client.connected */ {
    //   return Err(AuthError::ConnectionFailed("client is offline".to_string()));
    // }

    // let response = client.login(self.username.clone(), self.password.clone())
    //   .map_err(|e| AuthError::Unexpected(e.to_string()))?;

    // status
    // if true {
    //   return Err(AuthError::InvalidCredentials);
    // }




    response.access_token = "".to_string();


    self.access_token = response.access_token.clone();
    self.refresh_token = response.refresh_token.clone();
    Ok(response)
  }

  pub fn get_access_token(&mut self) -> String { self.access_token.clone() }
  pub fn get_refresh_token(&mut self) -> String { self.refresh_token.clone() }

}
