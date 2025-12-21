use crate::belling::utils::json::data::Data;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entity{
  pub data: Data
}

impl Entity {
  pub fn new() -> Self{
    Entity{
      data: Data::new()
    }
  }
}

