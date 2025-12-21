#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comprobante{
  pub request_uuid: String,
  pub encode: String
}

impl Comprobante {
  pub fn new()->Self{ Comprobante{
    request_uuid: String::new(),
    encode: String::new()
  } }
}


