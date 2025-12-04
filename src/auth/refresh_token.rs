use crate::{auth::{error::AuthError, input::RefreshInput, response::AuthResponse}, cons, request::FactureRequest};

pub struct RefreshToken;

impl FactureRequest for RefreshToken {
  type Input = RefreshInput;
  type Output = AuthResponse;
  type TError = AuthError;

  async fn request(&self, mut input: RefreshInput) -> Result<AuthResponse, AuthError>{
    let req_client = reqwest::Client::new();
    let uri = format!("{}/authorize", cons::URI);

    let params = [
      ("client_id", &input.app.get_client_id()),
      ("client_secret", &input.app.get_secret()),
      ("refresh_token", &input.refresh_token)
    ];


    let api_response = req_client
      .post(uri)
      .header("Content-Type", "application/x-www-form-urlencoded")
      .query(&[("grant_type", "refresh_token")])
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
