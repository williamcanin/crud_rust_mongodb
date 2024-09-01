// use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
// #[tokio::main]
// async fn main() -> mongodb::error::Result<()> {
//   let mut client_options =
//     ClientOptions::parse("mongodb+srv://<username>:<db_password>@cluster0.ofbyy.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0")
//       $.await?;
//   // Set the server_api field of the client_options object to set the version of the Stable API on the client
//   let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//   client_options.server_api = Some(server_api);
//   // Get a handle to the cluster
//   let client = Client::with_options(client_options)?;
//   // Ping the server to see if you can connect to the cluster
//   client
//     .database("admin")
//     .run_command(doc! {"ping": 1}, None)
//     .await?;
//   println!("Pinged your deployment. You successfully connected to MongoDB!");
//   Ok(())
// }

use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserFields {
    pub name: String,
    pub email: String,
}

pub struct ConnectionData {
    pub is_localhost: bool,
    pub username: &'static str,
    pub password: &'static str,
    pub url: &'static str,
    pub db_name: &'static str,
    pub cluster: &'static str,
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
            data.username, data.password, data.url, data.cluster
        );

        if data.is_localhost {
            uri = "mongodb://127.0.0.1:27017".to_string();
        }

        let client = Client::with_uri_str(uri).await?;
        let database = client.database(data.db_name);

        Ok(Connection { client, database })
    }

    pub fn get_collection(&self, name: &str) -> Collection<UserFields> {
        self.database.collection::<UserFields>(name)
    }
}
