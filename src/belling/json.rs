use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{belling::utils::{core::{error::BillingError, response::BillingResponse}, json::input::JsonInput}, core::{cons::URI, request::FactureRequest}
};

pub struct JsonBilling;

impl FactureRequest for JsonBilling{
  type Input = JsonInput;
  type Output = BillingResponse;
  type TError = BillingError;

  async fn request(&self, input: JsonInput) -> Result<BillingResponse, BillingError>{
    let req_client = reqwest::Client::new();
    let endpoint = format!("{}/timbrado/json", URI);

    let bearer = format!("Bearer {}", input.get_bearer());
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer.parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let json = input.get_json();

    let api_response = req_client
      .post(endpoint)
      .headers(headers)
      .query(&[("grant_type", "password")])
      .json(&json)
      .send()
      .await?;

    if api_response.status().is_success(){
      let json = api_response.text().await?;
      let response: BillingResponse = serde_json::from_str(json.as_str()).unwrap();
      return Ok(response);
    }
    else{
      return Err(
        BillingError::Api {
          status: api_response.status().as_u16() ,
          message: api_response.text().await.unwrap()
        }
      )
    }

  }


}

