use crate::structs::Connection;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::ClientOptions,
    Client,
};
use serde::de::DeserializeOwned;

// Enumeração para filtros
#[allow(dead_code)]
pub enum Filter {
    Document(Document),
    ObjectId(ObjectId),
}

#[allow(dead_code)]
impl Filter {
    // Converte a enumeração para mongodb::bson::Document
    fn to_document(&self) -> Document {
        match self {
            // Adiciona dois tipos de valores para o enum que serão usados
            // para tipos de pesquisa diferente
            Filter::Document(doc) => doc.clone(),
            Filter::ObjectId(id) => doc! { "_id": id.clone() },
        }
    }
}

#[allow(dead_code)]
pub async fn data<'a, T>(con: Connection<'a>, filter: Filter) -> mongodb::error::Result<Vec<T>>
where
    T: DeserializeOwned, // Garante que T pode ser desserializado a partir de BSON
    T: Send + Sync + 'static, // Adiciona requisitos para async
{
    // Configura as opções do cliente MongoDB
    let client_options = ClientOptions::parse(con.url).await?;

    // Conecta ao cliente MongoDB
    let client = Client::with_options(client_options)?;

    // Acessa o banco de dados e a coleção
    let db = client.database(con.db_name);
    let collection = db.collection::<T>(con.collection);

    // Converte o filtro para mongodb::bson::Document
    let filter_doc = filter.to_document();

    // Busca os documentos
    let mut cursor = collection.find(filter_doc).await?;

    // Coleta os resultados
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => results.push(doc),
            Err(e) => return Err(e.into()), // Retorna o erro se ocorrer
        }
    }

    Ok(results)
}
