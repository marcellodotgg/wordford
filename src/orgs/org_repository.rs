use sqlx::SqlitePool;

pub struct OrgRepository {
    db: SqlitePool,
}

impl OrgRepository {
    pub fn new(db: SqlitePool) -> Self {
        OrgRepository { db }
    }
}
