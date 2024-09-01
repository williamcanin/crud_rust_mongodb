use dotenv::dotenv;
use std::env;

use crate::database::ConnectionData;
// Url de conexão local (localhost).
pub const DEVELOPMENT_URL: &str = "mongodb://127.0.0.1:27017";

// Dados de conexão remota.
pub fn connection_data() -> ConnectionData {
    // Carrega as variáveis do arquivo .env
    dotenv().ok();
    // Set as variáveis do arquivo .env
    ConnectionData {
        development: false,
        username: env::var("MONGODB_USERNAME").unwrap(),
        hostname: env::var("MONGODB_HOSTNAME").unwrap(),
        password: env::var("MONGODB_PASSWORD").unwrap(),
        db_name: env::var("MONGODB_DB_NAME").unwrap(),
        cluster: env::var("MONGODB_CLUSTER").unwrap(),
    }
}
