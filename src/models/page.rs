use crate::models::{app::App, content::Content};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageWithContent {
    pub app: App,
    pub page: Page,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewPageRequest {
    pub app_id: i64,
    pub name: String,
}

pub type PageContent = HashMap<String, String>;
