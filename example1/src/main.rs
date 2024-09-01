mod delete;
mod insert;
mod search;
mod structs;
mod update;

use structs::Connection;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let con = Connection {
        url: "mongodb://localhost:27017",
        db_name: "mydb",
        collection: "mycollection",
    };

    // // --------- Inserir documento --------
    // use std::collections::HashMap;
    // let map = HashMap::from([("name", "William da Costa Canin"), ("city", "São Paulo")]);
    // let id = insert::data(con, map).await?;
    // println!("Documento inserido com ID: {:?}", id);

    // // --------- Pesquisar documento --------
    // // Exemplo de filtro com ObjectId
    // use crate::search::Filter;
    // use mongodb::bson::oid::ObjectId;
    // let object_id = ObjectId::parse_str("66d2528b100145b8989eb2f4")?;
    // let filter = Filter::ObjectId(object_id);

    // // // Exemplo de filtro por "name"
    // use crate::search::Filter;
    // // use mongodb::bson::doc;
    // // let filter = Filter::Document(doc! {"name": "Will"});

    // use structs::MyDocument;
    // let documents: Vec<MyDocument> = search::data(con, filter).await?;

    // for doc in documents {
    //     println!("{}", doc.name);
    // }

    // // ----------- Atualizar documento -----------
    // use mongodb::bson::oid::ObjectId;
    // use std::collections::HashMap;
    // let map = HashMap::from([("name", "William Canin")]);
    // let id = ObjectId::parse_str("66d21ff72840b47287fa4fd3")?;
    // update::data(con, id, map).await?;

    // ----------- Deleta documento -----------
    use mongodb::bson::oid::ObjectId;
    let id = ObjectId::parse_str("66d2528b100145b8989eb2f4")?;
    delete::a_record(con, id).await?;

    Ok(())
}
