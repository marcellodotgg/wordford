use crate::models::{
    app::{App, AppWithPages, CreateAppForm},
    page::Page,
};
use sqlx::SqlitePool;

pub struct AppRepository {
    db: SqlitePool,
}

impl AppRepository {
    pub fn new(db: &SqlitePool) -> Self {
        AppRepository { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: &i64) -> Result<AppWithPages, sqlx::Error> {
        let app = sqlx::query!(
            r#"
            SELECT * FROM apps WHERE id = ?
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        let pages = sqlx::query!(
            r#"
            SELECT * FROM pages WHERE app_id = ?
            "#,
            app.id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(AppWithPages {
            app: App {
                id: app.id,
                name: app.name,
                description: app.description.unwrap_or("".to_string()),
                url: app.url.unwrap_or("".to_string()),
                created_at: app.created_at.to_string(),
                updated_at: app.updated_at.to_string(),
            },
            pages: pages
                .into_iter()
                .map(|p| Page {
                    id: p.id.expect("id should not be null"),
                    app_id: p.app_id,
                    name: p.name,
                    created_at: p.created_at.to_string(),
                    updated_at: p.updated_at.to_string(),
                })
                .collect(),
        })
    }

    pub async fn search(&self, search_str: &str) -> Result<Vec<App>, sqlx::Error> {
        let pattern = format!("%{}%", search_str);
        let apps = sqlx::query!(
            r#"SELECT * FROM apps WHERE LOWER(name) LIKE LOWER(?)"#,
            pattern
        )
        .fetch_all(&self.db)
        .await?;

        Ok(apps
            .into_iter()
            .map(|app| App {
                id: app.id,
                name: app.name,
                description: app.description.unwrap_or("".to_string()),
                url: app.url.unwrap_or("".to_string()),
                created_at: app.created_at.to_string(),
                updated_at: app.updated_at.to_string(),
            })
            .collect())
    }

    pub async fn find_pages_by_app_id(&self, app_id: &str) -> Result<Vec<Page>, sqlx::Error> {
        let pages = sqlx::query!(
            r#"
            SELECT * FROM pages WHERE app_id = ?
            "#,
            app_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(pages
            .into_iter()
            .map(|p| Page {
                id: p.id.expect("id should not be null"),
                app_id: p.app_id,
                name: p.name,
                created_at: p.created_at.to_string(),
                updated_at: p.updated_at.to_string(),
            })
            .collect())
    }

    pub async fn create_app(&self, request: CreateAppForm) -> Result<App, sqlx::Error> {
        let app = sqlx::query!(
            r#"
            INSERT INTO apps (name, description, url) VALUES (?, ?, ?)
            RETURNING *
            "#,
            request.name,
            request.description,
            request.url
        )
        .fetch_one(&self.db)
        .await?;

        Ok(App {
            id: app.id,
            name: request.name,
            description: request.description,
            url: request.url,
            created_at: app.created_at.to_string(),
            updated_at: app.updated_at.to_string(),
        })
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
