use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};

use crate::{
  core::{cons, request::FactureRequest},
  sucursal::utils::{error::SucursalError, input::SucursalInput, response::SucursalResponse}
};

pub struct Sucursal;

impl FactureRequest for Sucursal{
  type Input = SucursalInput;
  type Output = SucursalResponse;
  type TError = SucursalError;

  async fn request(&self, input: SucursalInput) -> Result<SucursalResponse, SucursalError>{
    let req_client = reqwest::Client::new();
    let endpoint = format!("{}/authorize", cons::URI);

    let bearer = format!("Bearer {}", input.bearer);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer.parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let api_response = req_client
      .post(endpoint)
      .headers(headers)
      .query(&input.filters.list)
      .send()
      .await?;


    // Return ok when the status is successful
    if api_response.status().is_success(){
      let json = api_response.text().await?;
      let response: SucursalResponse = serde_json::from_str(json.as_str()).unwrap();
      return Ok(response);
    }

    // else return error result
    else{
      return Err(SucursalError::Api {
        status: api_response.status().as_u16() ,
        message: api_response.text().await.unwrap()
      })
    }

  }


}

