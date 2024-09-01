mod create;
mod delete;
mod read;
mod structs;
mod update;

use mongodb::Client;
use tokio;

use crate::create::create_user;
use crate::delete::delete_user;
use crate::read::read_user;
use crate::update::update_user;
use structs::{Connection, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let con = Connection {
        url: "mongodb://localhost:27017",
        db: "test_db",
        collection: "users",
    };

    let client = Client::with_uri_str(con.url).await?;
    let db = client.database(con.db);
    let collection = db.collection::<User>(con.collection);

    // Exemplo de uso
    let new_user = User {
        name: String::from("William"),
        email: String::from("william@example.com"),
    };

    let id = create_user(&collection, new_user).await?;

    read_user(&collection, &id).await?;

    let updated_user = User {
        name: String::from("William Canin"),
        email: String::from("william.canin@example.com"),
    };

    update_user(&collection, &id, updated_user).await?;

    delete_user(&collection, &id).await?;

    Ok(())
}
