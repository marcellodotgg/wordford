use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

pub mod content_repository;
pub mod content_service;
pub mod content_routes;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentDAO {
    id: i64,
    page_name: String,
    content: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: String,
    pub content: String,
}