use sqlx::SqlitePool;

pub mod content;

pub struct AppState {
    pub db: SqlitePool,
}