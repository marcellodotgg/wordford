use std::sync::Arc;

use sqlx::SqlitePool;
use tera::Tera;

pub mod api;
pub mod routes;

pub struct AppState {
    pub db: SqlitePool,
    pub tera: Arc<Tera>,
}
