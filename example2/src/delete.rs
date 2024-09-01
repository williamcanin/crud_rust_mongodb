use crate::structs::User;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

pub async fn delete_user(
    collection: &Collection<User>,
    id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = ObjectId::parse_str(id).unwrap();
    let delete = collection.delete_one(doc! { "_id": id }).await;
    match delete {
        Ok(_) => println!("Usuário deletado com sucesso!"),
        Err(err) => eprint!("Erro ao deletar: {}", err),
    }
    Ok(())
}
