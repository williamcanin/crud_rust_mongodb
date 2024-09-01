use crate::options::DEVELOPMENT_URL;
use mongodb::{Client, Collection, Database};
use std::marker::{Send, Sync};

pub struct ConnectionData {
  pub development: bool,
  pub username: String,
  pub password: String,
  pub hostname: String,
  pub db_name: String,
  pub cluster: String,
}

#[allow(dead_code)]
pub struct Connection {
  client: Client,
  database: Database,
}

impl Connection {
  pub async fn new(data: ConnectionData) -> Result<Self, Box<dyn std::error::Error>> {
    let mut uri = format!(
      "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName={}",
      data.username, data.password, data.hostname, data.cluster
    );

    if data.development {
      uri = DEVELOPMENT_URL.to_string();
    }

    let client = Client::with_uri_str(uri).await?;
    let database = client.database(&data.db_name);

    Ok(Connection { client, database })
  }

  pub fn get_collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
    self.database.collection::<T>(name)
  }
}
