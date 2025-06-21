use sqlx::SqlitePool;

use crate::models::content::{Content, NewContentRequest};

pub struct ContentRepository {
    db: SqlitePool,
}

impl ContentRepository {
    pub fn new(db: SqlitePool) -> Self {
        ContentRepository { db }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Content, sqlx::Error> {
        let content = sqlx::query!(
            r#"
            SELECT * FROM content WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Content {
            id: content.id,
            page_id: content.page_id,
            name: content.name,
            body: content.body,
            created_at: content.created_at.to_string(),
            updated_at: content.updated_at.to_string(),
        })
    }

    pub async fn find_all_by_page_id(&self, page_id: i64) -> Result<Vec<Content>, sqlx::Error> {
        let contents = sqlx::query!(
            r#"
            SELECT * FROM content WHERE page_id = ?
            "#,
            page_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(contents
            .into_iter()
            .map(|c| Content {
                id: c.id.expect("id should not be null"),
                page_id: c.page_id,
                name: c.name,
                body: c.body,
                created_at: c.created_at.to_string(),
                updated_at: c.updated_at.to_string(),
            })
            .collect())
    }

    pub async fn create_content(
        &self,
        request: &NewContentRequest,
    ) -> Result<Content, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO content (page_id, name, body) VALUES (?, ?, ?)
            RETURNING id, page_id, name, body, created_at, updated_at
            "#,
            request.page_id,
            request.name,
            request.body
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Content {
            id: result.id.expect("id should not be null"),
            page_id: result.page_id,
            name: result.name,
            body: result.body,
            created_at: result.created_at.to_string(),
            updated_at: result.updated_at.to_string(),
        })
    }

    pub async fn delete_content(&self, id: &i64) -> Result<(), sqlx::Error> {
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
