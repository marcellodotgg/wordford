use sqlx::SqlitePool;

// TODO(Marcello): add this to swagger

pub struct PageRepository {
    db: SqlitePool,
}

impl PageRepository {
    pub fn new(db: SqlitePool) -> Self {
        PageRepository { db }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        let page = sqlx::query!(
            r#"
            SELECT * FROM pages WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(page.map(|p| p.name)) // TODO: should return a Page struct instead
    }

    pub async fn create_page(&self, app_id: &str, name: &str) -> Result<String, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO pages (app_id, name) VALUES (?, ?)
            "#,
            app_id,
            name
        )
        .execute(&self.db)
        .await?;

        Ok("success".to_string()) // TODO: should return the created page ID
    }

    pub async fn delete_page(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM pages WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
