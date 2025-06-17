use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub mod content_repository;
pub mod content_routes;
pub mod content_service;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDAO {
    id: i64,
    page_name: String,
    content: String,
    content_id: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub page_name: String,
    pub content_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateContentRequest {
    pub content_id: String,
    pub content: String,
}
