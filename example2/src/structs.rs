use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub struct Connection {
    pub url: &'static str,
    pub db: &'static str,
    pub collection: &'static str,
}
