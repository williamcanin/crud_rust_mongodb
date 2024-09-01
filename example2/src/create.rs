use crate::structs::User;
use mongodb::Collection;

pub async fn create_user(
    collection: &Collection<User>,
    user: User,
) -> Result<String, Box<dyn std::error::Error>> {
    let insert_result = collection.insert_one(user.to_owned()).await?;

    // Obtém o ID do documento inserido
    let get_id = match insert_result.inserted_id.as_object_id() {
        Some(id) => {
            println!("Usuário \"{}\" inserido com sucesso!", user.name);
            id.to_hex()
        }
        None => {
            eprintln!("Erro ao criar usuário.");
            return Err("Erro ao criar usuário.".into());
        }
    };
    Ok(get_id)
}
