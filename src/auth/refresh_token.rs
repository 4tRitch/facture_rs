use crate::{
  auth::utils::{error::AuthError, input::RefreshInput, response::AuthResponse},
  core::{cons, request::FactureRequest}
};

pub struct RefreshToken;

impl FactureRequest for RefreshToken {
  type Input = RefreshInput;
  type Output = AuthResponse;
  type TError = AuthError;

  async fn request(&self, mut input: RefreshInput) -> Result<AuthResponse, AuthError>{
    let req_client = reqwest::Client::new();
    let endpoint = format!("{}/authorize", cons::URI);

    let params = [
      ("client_id", &input.app.get_client_id()),
      ("client_secret", &input.app.get_client_secret()),
      ("refresh_token", &input.refresh_token)
    ];


    let api_response = req_client
      .post(endpoint)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .query(&[("grant_type", "refresh_token")])
      .form(&params)
      .send()
      .await?;


    // Return ok when the status is successful
    if api_response.status().is_success(){
      let json = api_response.text().await?;
      let response: AuthResponse = serde_json::from_str(json.as_str()).unwrap();
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
