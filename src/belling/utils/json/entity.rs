use crate::belling::utils::json::data::Data;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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

#[derive(Serialize)]
pub struct EntityWrapper<'a>{
  pub entity: &'a Entity,
}

