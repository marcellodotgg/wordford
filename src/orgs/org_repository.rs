use sqlx::SqlitePool;

pub struct OrgRepository {
    db: SqlitePool,
}

impl OrgRepository {
    pub fn new(db: SqlitePool) -> Self {
        OrgRepository { db }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        let org = sqlx::query!(
            r#"
            SELECT name FROM orgs WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(org.map(|o| o.name)) // TODO: should return an Org struct instead
    }

    pub async fn create_org(&self, name: &str) -> Result<String, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO orgs (name) VALUES (?)
            "#,
            name
        )
        .execute(&self.db)
        .await?;

        Ok("success".to_string()) // TODO: should return the created org ID
    }

    pub async fn delete_org(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM orgs WHERE id = ?
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
