use sqlx::SqlitePool;

pub mod apps;
pub mod content;
pub mod pages;

pub struct AppState {
    pub db: SqlitePool,
}
