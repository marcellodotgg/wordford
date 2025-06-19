use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::content::Content;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Page {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct PageWithContent {
    pub page: Page,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewPageRequest {
    pub app_id: i64,
    pub name: String,
}

pub type PageContent = HashMap<String, String>;
