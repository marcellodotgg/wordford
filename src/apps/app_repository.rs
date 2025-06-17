use sqlx::SqlitePool;

pub struct AppRepository {
    db: SqlitePool,
}

impl AppRepository {
    pub fn new(db: SqlitePool) -> Self {
        AppRepository { db }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        let app = sqlx::query!(
            r#"
            SELECT name FROM apps WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(app.map(|a| a.name)) // TODO: should return an App struct instead
    }

    pub async fn create_app(&self, name: &str) -> Result<String, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO apps (name) VALUES (?)
            "#,
            name
        )
        .execute(&self.db)
        .await?;

        Ok("success".to_string()) // TODO: should return the created app ID
    }

    pub async fn delete_app(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM apps WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
