use sqlx::SqlitePool;

use crate::pages::{NewPageRequest, Page, PageContent, PageWithContent};

pub struct PageRepository {
    db: SqlitePool,
}

impl PageRepository {
    pub fn new(db: SqlitePool) -> Self {
        PageRepository { db }
    }

    pub async fn find_by_id(&self, id: &i64) -> Result<Option<PageWithContent>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
            p.id as "id!",
            p.app_id as "app_id!",
            p.name as "name!",
            p.created_at as "created_at!",
            p.updated_at as "updated_at!",
            c.name as "content_name?",
            c.body as "content_body?"
            FROM pages p
            LEFT JOIN content c ON p.id = c.page_id
            WHERE p.id = ?
            "#,
            id
        )
        .fetch_all(&self.db)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let first = &rows[0];
        let mut content = std::collections::HashMap::new();

        for row in &rows {
            if let (Some(name), Some(body)) = (row.content_name.clone(), row.content_body.clone()) {
                content.insert(name, body);
            }
        }

        Ok(Some(PageWithContent {
            page: Page {
                id: first.id,
                app_id: first.app_id,
                name: first.name.clone(),
                created_at: first.created_at.to_string(),
                updated_at: first.updated_at.to_string(),
            },
            content,
        }))
    }

    pub async fn get_content_for_page(&self, page_id: &i64) -> Result<PageContent, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM content
            WHERE page_id = ? 
            "#,
            page_id
        )
        .fetch_all(&self.db)
        .await?;

        let rows = rows.into_iter().map(|row| (row.name, row.body)).collect();

        Ok(rows)
    }

    pub async fn create_page(&self, page: NewPageRequest) -> Result<Page, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO pages (app_id, name) VALUES (?, ?)
            RETURNING id, app_id, name, created_at, updated_at
            "#,
            page.app_id,
            page.name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Page {
            id: record.id.expect("id should not be null"),
            app_id: record.app_id,
            name: record.name,
            created_at: record.created_at.to_string(),
            updated_at: record.updated_at.to_string(),
        })
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
