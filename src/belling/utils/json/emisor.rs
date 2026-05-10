#[derive(serde::Deserialize, serde::Serialize)]
pub struct Emisor{
  pub id: usize
}

impl Emisor {
  pub fn new() -> Self {
    Emisor{
      id: 0
    }
  }
}
