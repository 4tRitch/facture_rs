#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonInput {
  json: JsonRaw,
  bearer: String
}

impl JsonInput {
  pub fn new() -> Self{
    JsonInput{
      bearer: String::new(),
      json: JsonRaw::new()
    }
  }

  pub fn set_comprobante(mut self, comprobante: impl Into<Comprobantes>) -> Self{
    self.json.entity.data.comprobantes.push(comprobante.into());
    self
  }

  pub fn set_sucursal(mut self, id: impl Into<usize>) -> Self{
    self.json.entity.data.sucursal.id = id.into();
    self
  }

  pub fn set_bearer(mut self, bearer: impl Into<String>) -> Self{
    self.bearer = bearer.into();
    self
  }

  pub fn get_bearer(&self) -> String{
    self.bearer.clone()
  }

  pub fn get_json(&self) -> String {
    "".to_string()
  }

}



