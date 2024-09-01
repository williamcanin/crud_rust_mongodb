mod users;
use crate::users::controller::UserController;
use crate::users::model::{Connection, ConnectionData, UserFields};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Exemplo de uri:
    // localhost: "mongodb://localhost:27017";
    // remoto: "mongodb+srv://<username>:<db_password>@cluster0.ofbyy.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"

    let con = ConnectionData {
        is_localhost: false,
        username: "username",
        password: "db_password",
        url: "cluster0.ofbyy.mongodb.net",
        db_name: "mydb",
        cluster: "Cluster0",
    };

    let db = Connection::new(con).await?;
    let collection = UserController::new(db.get_collection("users"));

    let new_user = UserFields {
        name: "William".to_string(),
        email: "william@example.com".to_string(),
    };

    let user_id = collection.create_user(new_user).await?;
    println!("User created with ID: {}", user_id);

    let user = collection.read_user(&user_id.to_hex()).await?;
    if let Some(user) = user {
        println!("User details: {:?}", user);
    } else {
        println!("User not found");
    }

    let updated_user = UserFields {
        name: "William Canin".to_string(),
        email: "william.canin@example.com".to_string(),
    };
    collection
        .update_user(&user_id.to_hex(), updated_user)
        .await?;

    // collection.delete_user(&user_id.to_hex()).await?;

    Ok(())
}
