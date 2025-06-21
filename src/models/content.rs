use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: i64,
    pub page_id: i64,
    pub name: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewContentRequest {
    pub page_id: i64,
    pub name: String,
    pub body: String,
}
