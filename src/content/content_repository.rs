use sqlx::SqlitePool;

pub struct ContentRepository {
    db: SqlitePool,
}

impl ContentRepository {
    pub fn new(db: SqlitePool) -> Self {
        ContentRepository { db }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        let content = sqlx::query!(
            r#"
            SELECT * FROM content WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(content.map(|c| c.body))
    }

    pub async fn create_content(&self, page_id: &str, name: &str) -> Result<String, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO content (page_id, name) VALUES (?, ?)
            "#,
            page_id,
            name
        )
        .execute(&self.db)
        .await?;

        Ok("success".to_string()) // TODO: should return the created content ID
    }

    pub async fn delete_content(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM content WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
