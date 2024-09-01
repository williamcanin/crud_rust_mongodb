use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserFields {
  pub name: String,
  pub email: String,
  pub country: String,
}
