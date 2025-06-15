use sqlx::SqlitePool;
use crate::content::Content;

pub struct ContentRepository {
    db: SqlitePool,
}

impl ContentRepository {
    pub fn new(db: SqlitePool) -> Self {
        ContentRepository { db }
    }
    pub async fn get_content(&self, page_name: &str) -> Result<Vec<Content>, sqlx::Error> {
        let content = sqlx::query_as!(
            Content,
            r#"
            SELECT id, page_name, content, created_at, updated_at
            FROM content WHERE page_name = ?
            "#,
            page_name
        )
        .fetch_all(&self.db)
        .await?;
        Ok(content)
    }
}