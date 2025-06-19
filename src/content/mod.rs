use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod content_repository;
pub mod content_routes;
pub mod content_service;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Content {
    pub id: i64,
    pub page_id: i64,
    pub name: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewContentRequest {
    pub page_id: i64,
    pub name: String,
    pub body: String,
}
