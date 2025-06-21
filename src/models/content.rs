use crate::models::page::FullPage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FindContentByPageIdParams {
    pub page_id: i64,
}

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
pub struct FullContent {
    pub content: Content,
    pub page: FullPage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewContentRequest {
    pub page_id: i64,
    pub name: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateContentRequest {
    pub content_id: i64,
    pub name: String,
    pub body: String,
}
