use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BillingResponse{
  pub success: bool,
  pub code: usize,
  pub result: ResultData,
  pub message: String
}


#[derive(Deserialize, Debug, Clone)]
pub struct ResultData{
  pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
  #[serde(rename = "requestUuid")]
  pub request_uuid: String,
  pub encode: String,
  pub succeed: bool,
  pub uuid: String,
  pub message: String,
}
