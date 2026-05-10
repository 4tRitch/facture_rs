use crate::{
  auth::utils::{error::AuthError, input::PasswordInput, response::AuthResponse},
  core::{cons, request::FactureRequest}
};

pub struct Password;

impl FactureRequest for Password {
  type Input = PasswordInput;
  type Output = AuthResponse;
  type TError = AuthError;

  async fn request(&self, mut input: PasswordInput) -> Result<AuthResponse, AuthError>{
    let req_client = reqwest::Client::new();
    let endpoint = format!("{}/authorize", cons::URI);

    let params = [
      ("username", &input.app.credentials.get_user()),
      ("password", &input.app.credentials.get_password()),
      ("scope", &input.scopes),
      ("client_id", &input.app.get_client_id()),
      ("client_secret", &input.app.get_client_secret())
    ];


    let api_response = req_client
      .post(endpoint)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .query(&[("grant_type", "password")])
      .form(&params)
      .send()
      .await?;


    // Return ok when the status is successful
    if api_response.status().is_success(){
      let json = api_response.text().await?;
      let response: AuthResponse = serde_json::from_str(&json).unwrap();
      return Ok(response);
    }

    // else return error result
    else{
      return Err(AuthError::Api {
        status: api_response.status().as_u16() ,
        message: api_response.text().await.unwrap()
      })
    }

  }


}
