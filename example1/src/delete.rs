use crate::structs::Connection;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::ClientOptions,
    Client,
};

#[allow(dead_code)]
pub async fn a_record<'a>(con: Connection<'a>, id: ObjectId) -> mongodb::error::Result<()> {
    // Configura as opções do cliente MongoDB
    let client_options = ClientOptions::parse(con.url).await?;

    // Conecta ao cliente MongoDB
    let client = Client::with_options(client_options)?;

    // Acessa o banco de dados e a coleção
    let db = client.database(con.db_name);
    let collection = db.collection::<Document>(con.collection);

    let filter = doc! { "_id": id };

    // Deleta um documento
    let delete_result = collection.delete_one(filter).await?;

    if delete_result.deleted_count == 0 {
        eprintln!("Nenhum documento encontrado para deletar");
    }

    Ok(())
}
