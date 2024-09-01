use crate::structs::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

pub async fn update_user(
    collection: &Collection<User>,
    id: &str,
    user: User,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(id)?;
    let update = collection
        .update_one(
            doc! { "_id": id },
            doc! { "$set": { "name": user.name, "email": user.email } },
        )
        .await;
    match update {
        Ok(_) => println!("Atualização realizada com sucesso!"),

        Err(err) => eprint!("Erro ao atualizar: {}", err),
    }
    Ok(())
}
