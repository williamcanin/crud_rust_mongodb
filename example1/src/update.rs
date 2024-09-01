use std::collections::HashMap;

use crate::structs::Connection;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    options::ClientOptions,
    Client,
};

// Insert and update at the same time, getting the id.
#[allow(dead_code)]
pub async fn data<'a, T: Into<Bson>>(
    con: Connection<'a>,
    id: ObjectId,
    map: HashMap<&str, T>,
) -> mongodb::error::Result<()> {
    // Configura as opções do cliente MongoDB
    let client_options = ClientOptions::parse(con.url).await?;

    // Conecta ao cliente MongoDB
    let client = Client::with_options(client_options)?;

    // Acessa o banco de dados e a coleção
    let db = client.database(con.db_name);
    let collection = db.collection::<Document>(con.collection);

    // Agora você pode usar o `id` para futuras operações, como uma atualização
    let mut doc = Document::new();
    for (key, value) in map {
        doc.insert(key, value.into());
    }
    let filter = doc! { "_id": id };
    let update = doc! { "$set": doc };

    // Atualiza o documento
    let update_result = collection.update_one(filter, update).await?;
    println!(
        "Número de documentos atualizados: {:?}",
        update_result.modified_count
    );

    Ok(())
}
