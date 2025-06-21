use serde::{Deserialize, Serialize};

use crate::models::page::Page;

#[derive(Deserialize, Serialize, Debug)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppWithPages {
    pub app: App,
    pub pages: Vec<Page>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppSearch {
    pub name: String,
}
