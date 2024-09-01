use mongodb::bson::oid::ObjectId;

// Define a struct MyDocument com o campo `_id` como `ObjectId` do MongoDB
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct MyDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}

pub struct Connection<'a> {
    pub url: &'a str,
    pub db_name: &'a str,
    pub collection: &'a str,
}
