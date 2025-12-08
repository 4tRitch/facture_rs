use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
  #[error("network error: {0}")]
  Network(#[from] reqwest::Error),

  #[error("parse error: {0}")]
  Parse(#[from] serde_json::Error),

  #[error("api error {status}: {message}")]
  Api {
    status: u16,
    message: String,
  },
}
