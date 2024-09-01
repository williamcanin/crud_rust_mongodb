use crate::structs::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

pub async fn read_user(
    collection: &Collection<User>,
    user_id: &str,
) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(user_id)?;

    let user = collection.find_one(doc! { "_id": id }).await?;

    // Verifica se o usuário foi encontrado
    if let Some(result) = user {
        println!("Usuário encontrado: {}", result.name);
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
