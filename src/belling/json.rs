use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{belling::utils::{core::{enums::BillingType, error::BillingError, response::BillingResponse}, json::input::JsonInput}, core::{cons::URI, request::FactureRequest}
};

pub struct JsonBilling;

impl FactureRequest for JsonBilling{
  type Input = JsonInput;
  type Output = BillingResponse;
  type TError = BillingError;

  async fn request(&self, input: JsonInput) -> Result<BillingResponse, BillingError>{
    let req_client = reqwest::Client::new();
    let endpoint = if input.bill_type == BillingType::JSON  {format!("{}/timbrado/json", URI)} else {format!("{}/timbrado/", URI)} ;

    let bearer = format!("Bearer {}", input.get_bearer());
    let mut headers = HeaderMap::new();
    let auth_header = bearer
      .parse::<HeaderValue>()
      .map_err(|error| BillingError::RequestSetup(format!("invalid authorization header: {error}")))?;
    headers.insert(AUTHORIZATION, auth_header);
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let json = input.get_json()?;

    let api_response = req_client
      .post(endpoint)
      .headers(headers)
      .query(&[("grant_type", "password")])
      .body(json)
      .send()
      .await?;

    if api_response.status().is_success(){
      let json = api_response.text().await?;
      println!("{}",json);
      let response: BillingResponse = serde_json::from_str(&json)?;
      return Ok(response);
    }
    else{
      let status = api_response.status().as_u16();
      let message = api_response
        .text()
        .await
        .unwrap_or_else(|error| format!("failed to read api error body: {error}"));

      return Err(
        BillingError::Api {
          status,
          message
        }
      )
    }
  }

}
