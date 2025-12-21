use crate::belling::utils::json::entity::Entity;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonRaw{
  pub entity: Entity
}

impl JsonRaw {
  pub fn new() -> Self{
    JsonRaw{
      entity: Entity::new()
    }
  }
}
