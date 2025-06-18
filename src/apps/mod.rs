use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::pages::Page;

pub mod app_repository;
pub mod app_routes;
pub mod app_service;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct App {
    pub id: i64,
    pub org_id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct AppWithPages {
    pub app: App,
    pub pages: Vec<Page>,
}
