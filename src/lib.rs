use std::sync::Arc;

use sqlx::SqlitePool;
use tera::Tera;

pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod user;

pub struct AppState {
    pub db: SqlitePool,
    pub tera: Arc<Tera>,
}
