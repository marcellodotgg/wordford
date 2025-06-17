use sqlx::SqlitePool;

pub mod apps;
pub mod content;

pub struct AppState {
    pub db: SqlitePool,
}
