use std::collections::HashMap;

use sqlx::SqlitePool;
use crate::content::Content;

pub struct ContentRepository {
    db: SqlitePool,
}

impl ContentRepository {
    pub fn new(db: SqlitePool) -> Self {
        ContentRepository { db }
    }

    pub async fn get_content(&self, page_name: &str) -> Result<HashMap<String, String>, sqlx::Error> {
        let content = sqlx::query_as!(
            Content,
            r#"
            SELECT page_name || '_' || CAST(id AS TEXT) as id, content 
            FROM content WHERE page_name = ?
            "#,
            page_name
        )
        .fetch_all(&self.db)
        .await?;

        if content.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(content.into_iter().map(|c| (c.id, c.content)).collect())
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
}