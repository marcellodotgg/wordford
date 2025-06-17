use sqlx::SqlitePool;

pub struct AppRepository {
    db: SqlitePool,
}

impl AppRepository {
    pub fn new(db: SqlitePool) -> Self {
        AppRepository { db }
    }
}
