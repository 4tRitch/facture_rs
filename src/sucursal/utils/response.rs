use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SucursalResponse{
  pub success: bool,
  pub code: usize,
  pub pagination: ResultData,
  pub message: String
}


#[derive(Deserialize, Debug, Clone)]
pub struct ResultData{
  pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
  pub id: usize,
  pub nombre: String,
  pub direccion: Direction
}


#[derive(Debug, Deserialize, Clone)]
pub struct Direction{
  pub id: usize,
  pub nombre: String,

  #[serde(rename = "catCodigoPostal")]
  pub cat_zip_code: String,

  pub calle: String,

  #[serde(rename = "numerointerior")]
  pub internal_num: String,

  #[serde(rename = "numeroexterior")]
  pub external_num: String,

  #[serde(rename = "codigopostal")]
  pub zip_code: String,

  pub colonia: String,
  pub municipio: String,
  pub ciudad: String,
  pub estado: String,
  pub pais: String,
  pub referencia: String
}
