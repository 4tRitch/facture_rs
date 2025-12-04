use serde::{Deserialize, Serialize};

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

