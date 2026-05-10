use crate::belling::utils::{core::comprobante::Comprobante, json::{emisor::Emisor, sucursal::Sucursal}};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Data{
  pub comprobantes: Vec<Comprobante>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub template: Option<String>,
  pub emisor: Emisor,
  pub sucursal: Sucursal
}

impl Data {
  pub fn new() -> Self {
    Data{
      comprobantes: Vec::new(),
      template: None,
      emisor: Emisor::new(),
      sucursal: Sucursal::new()
    }
  }
}



