use crate::structs::Connection;
use mongodb::{
    bson::{oid::ObjectId, Bson, Document},
    options::ClientOptions,
    Client,
};
use rayon::prelude::*;
use std::collections::HashMap;

// Insert and update at the same time, getting the id.
#[allow(dead_code)]
pub async fn data<'a, T: Into<Bson> + Send + Sync + std::clone::Clone>(
    con: &Connection<'a>,
    map: HashMap<&str, T>,
) -> mongodb::error::Result<ObjectId> {
    // Configura as opções do cliente MongoDB
    let client_options = ClientOptions::parse(con.url).await?;

    // Conecta ao cliente MongoDB
    let client = Client::with_options(client_options)?;

    // Acessa o banco de dados e a coleção
    let db = client.database(con.db_name);
    let collection = db.collection::<Document>(con.collection);

    // Convertendo o HashMap em um vetor de pares e processando em paralelo com rayon
    let pairs: Vec<(String, Bson)> = map
        .par_iter() // Itera sobre o HashMap em paralelo
        .map(|(key, value)| (key.to_string(), value.into())) // Transforma as chaves e valores
        .collect(); // Coleta em um vetor de pares

    // Criando o Document a partir dos pares
    let new_document = Document::from_iter(pairs);

    // Modo comum de adicionar com for
    // let mut new_document = Document::new();
    // for (key, value) in map {
    //     new_document.insert(key, value.into());
    // }

    // Insere o documento e obtém o resultado
    let insert_result = collection.insert_one(new_document).await?;

    // Obtém o ID do documento inserido
    let get_id = match insert_result.inserted_id.as_object_id() {
        Some(id) => id,
        None => {
            eprintln!("O ID inserido não é um ObjectId");
            std::process::exit(1);
        }
    };

    Ok(get_id)
}
