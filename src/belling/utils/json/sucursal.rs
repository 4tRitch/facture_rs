#[derive(serde::Deserialize, serde::Serialize)]
pub struct Sucursal{
  pub id: usize
}

impl Sucursal {
  pub fn new() -> Self {
    Sucursal{
      id: 0
    }
  }
}
