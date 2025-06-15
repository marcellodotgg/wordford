use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

pub mod content_repository;
pub mod content_service;

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    id: i64,
    page_name: String,
    content: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}