use crate::belling::utils::{core::comprobante::Comprobante, json::sucursal::Sucursal};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Data{
  pub comprobantes: Vec<Comprobante>,
  pub sucursal: Sucursal
}

impl Data {
  pub fn new() -> Self {
    Data{
      comprobantes: Vec::new(),
      sucursal: Sucursal::new()
    }
  }
}



