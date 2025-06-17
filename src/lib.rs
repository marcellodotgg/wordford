use sqlx::SqlitePool;

pub mod apps;
pub mod content;
pub mod orgs;

pub struct AppState {
    pub db: SqlitePool,
}
