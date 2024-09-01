mod database;
mod options;
mod users;
use crate::options::connection_data;
use crate::users::controller::UserController;
use crate::users::model::UserFields;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Exemplo de uri:
    // localhost: "mongodb://localhost:27017";
    // remoto: "mongodb+srv://<username>:<db_password>@cluster0.ofbyy.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"

    let db = database::Connection::new(connection_data()).await?;
    let user_collection = db.get_collection::<UserFields>("users");
    let user_controler = UserController::new(user_collection);

    let new_user = UserFields {
        name: "William".to_string(),
        email: "william@example.com".to_string(),
    };

    let user_id = user_controler.create_user(new_user).await?;
    println!("User created with ID: {}", user_id);

    let user = user_controler.read_user(&user_id.to_hex()).await?;
    if let Some(user) = user {
        println!("User details: {:?}", user);
    } else {
        println!("User not found");
    }

    let updated_user = UserFields {
        name: "William Canin".to_string(),
        email: "william.canin@example.com".to_string(),
    };
    user_controler
        .update_user(&user_id.to_hex(), updated_user)
        .await?;

    // user_controler.delete_user(&user_id.to_hex()).await?;

    Ok(())
}
