use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BillingResponse{
  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub success: bool,

  #[serde(default, deserialize_with = "deserialize_usize_from_string_or_number")]
  pub code: usize,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub result: ResultData,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub message: String
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResultData{
  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Item {
  #[serde(rename = "requestUuid", default, deserialize_with = "deserialize_default_on_null")]
  pub request_uuid: String,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub encode: String,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub succeed: bool,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub uuid: String,

  #[serde(default, deserialize_with = "deserialize_default_on_null")]
  pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UsizeOrString {
  Usize(usize),
  String(String),
}

fn deserialize_usize_from_string_or_number<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
  D: Deserializer<'de>,
{
  let value = Option::<UsizeOrString>::deserialize(deserializer)?;

  match value {
    Some(UsizeOrString::Usize(value)) => Ok(value),
    Some(UsizeOrString::String(value)) => value.parse::<usize>().map_err(de::Error::custom),
    None => Ok(0),
  }
}

fn deserialize_default_on_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Deserialize<'de> + Default,
{
  Option::<T>::deserialize(deserializer).map(|value| value.unwrap_or_default())
}
