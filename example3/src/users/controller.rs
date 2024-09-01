use super::model::UserFields;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection};

use std::error::Error;

pub struct UserController {
    collection: Collection<UserFields>,
}

impl UserController {
    pub fn new(collection: Collection<UserFields>) -> Self {
        UserController { collection }
    }

    pub async fn create_user(&self, user: UserFields) -> Result<ObjectId, Box<dyn Error>> {
        let insert_result = self.collection.insert_one(user.clone()).await?;

        let get_id = match insert_result.inserted_id.as_object_id() {
            Some(id) => {
                println!("Usuário {} inserido com sucesso!", user.name);
                id
            }
            None => {
                return Err("O ID inserido não é um ObjectId".into());
            }
        };
        Ok(get_id)
    }

    pub async fn read_user(&self, id: &str) -> Result<Option<UserFields>, Box<dyn Error>> {
        let id = ObjectId::parse_str(id)?;

        let user = self.collection.find_one(doc! { "_id": id }).await?;

        if let Some(result) = user {
            println!("Usuário encontrado: {:?}", result);
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user(
        &self,
        id: &str,
        updated_user: UserFields,
    ) -> Result<(), Box<dyn Error>> {
        let id = ObjectId::parse_str(id)?;

        let result = self
            .collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name": updated_user.name.clone(), "email": updated_user.email } },
            )
            .await?;

        if result.matched_count > 0 {
            println!("Usuário {} atualizado com sucesso!", updated_user.name);
            Ok(())
        } else {
            Err("Nenhum usuário encontrado para atualizar".into())
        }
    }

    pub async fn delete_user(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let id = ObjectId::parse_str(id)?;

        let result = self.collection.delete_one(doc! { "_id": id }).await?;

        if result.deleted_count > 0 {
            println!("Usuário com ID {} deletado com sucesso!", id);
            Ok(())
        } else {
            Err("Nenhum usuário encontrado para deletar".into())
        }
    }
}
