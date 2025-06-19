use sqlx::Error::RowNotFound;
use sqlx::SqlitePool;

use crate::{
    api::content::Content,
    api::pages::{NewPageRequest, Page, PageContent, PageWithContent},
};

pub struct PageRepository {
    db: SqlitePool,
}

impl PageRepository {
    pub fn new(db: SqlitePool) -> Self {
        PageRepository { db }
    }

    pub async fn find_by_id(&self, id: &i64) -> Result<Option<PageWithContent>, sqlx::Error> {
        let page = sqlx::query!(
            r#"
            SELECT * FROM pages WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        let content = sqlx::query!(
            r#"
            SELECT * FROM content WHERE page_id = ?
            "#,
            id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(Some(PageWithContent {
            page: Page {
                id: page.id,
                app_id: page.app_id,
                name: page.name,
                created_at: page.created_at.to_string(),
                updated_at: page.updated_at.to_string(),
            },
            content: content
                .into_iter()
                .map(|c| Content {
                    id: c.id.expect("id should not be null"),
                    page_id: c.page_id,
                    name: c.name,
                    body: c.body,
                    created_at: c.created_at.to_string(),
                    updated_at: c.updated_at.to_string(),
                })
                .collect(),
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

        if rows.is_empty() {
            return Err(RowNotFound);
        }

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
