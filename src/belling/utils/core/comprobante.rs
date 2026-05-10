#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comprobante{
  #[serde(rename = "requestUuid")]
  pub request_uuid: String,
  pub encode: String
}

impl Comprobante {
  pub fn new()->Self{ Comprobante{
    request_uuid: String::new(),
    encode: String::new()
  } }
}


