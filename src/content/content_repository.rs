use crate::content::{Content, CreateContentRequest};
use sqlx::Error::RowNotFound;
use sqlx::SqlitePool;
use std::collections::HashMap;

pub struct ContentRepository {
    db: SqlitePool,
}

impl ContentRepository {
    pub fn new(db: SqlitePool) -> Self {
        ContentRepository { db }
    }

    pub async fn get_sitemap(&self) -> Result<Vec<String>, sqlx::Error> {
        let sitemap = sqlx::query!(
            r#"
            SELECT page_name FROM content GROUP BY page_name
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(sitemap.into_iter().map(|s| s.page_name).collect())
    }

    pub async fn get_content(
        &self,
        page_name: &str,
    ) -> Result<HashMap<String, String>, sqlx::Error> {
        let content = sqlx::query!(
            r#"
            SELECT content_id, content 
            FROM content WHERE page_name = ?
            "#,
            page_name
        )
        .fetch_all(&self.db)
        .await?;

        if content.is_empty() {
            return Err(RowNotFound);
        }

        Ok(content
            .into_iter()
            .map(|c| (c.content_id, c.content))
            .collect())
    }

    pub async fn create_content(
        &self,
        page_name: &str,
        request: CreateContentRequest,
    ) -> Result<Content, sqlx::Error> {
        let page_name = slug::slugify(page_name).replace("-", "_");
        let content_id = slug::slugify(&request.content_id).replace("-", "_");
        let content = sqlx::query_as!(
            Content,
            r#"
            INSERT INTO content (page_name, content_id, content) VALUES (?, ?, ?)
            RETURNING page_name, content_id, content
            "#,
            page_name,
            content_id,
            request.content
        )
        .fetch_one(&self.db)
        .await?;

        Ok(content)
    }

    pub async fn delete_content(
        &self,
        page_name: &str,
        content_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM content WHERE page_name = ? AND content_id = ?
            "#,
            page_name,
            content_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
