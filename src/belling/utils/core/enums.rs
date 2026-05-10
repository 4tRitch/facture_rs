#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Clone)]
pub enum BillingType{
  JSON,
  XML
}

