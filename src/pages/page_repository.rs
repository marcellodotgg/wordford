use sqlx::SqlitePool;

pub struct PageRepository {
    db: SqlitePool,
}

impl PageRepository {
    pub fn new(db: SqlitePool) -> Self {
        PageRepository { db }
    }
}
