use sqlx::SqlitePool;

use crate::{
    models::content::{Content, FullContent, NewContentRequest, UpdateContentRequest},
    repositories::{content::ContentRepository, pages::PageRepository},
};

pub struct ContentService {
    content_repository: ContentRepository,
    page_repository: PageRepository,
}

impl ContentService {
    pub fn new(db: &SqlitePool) -> Self {
        ContentService {
            content_repository: ContentRepository::new(&db),
            page_repository: PageRepository::new(&db),
        }
    }

    pub async fn find_by_id(&self, id: &i64) -> Result<Content, sqlx::Error> {
        self.content_repository.find_by_id(id).await
    }

    pub async fn full_content_by_id(&self, id: &i64) -> Result<FullContent, sqlx::Error> {
        let content = self.content_repository.find_by_id(id).await?;
        let page = self.page_repository.find_by_id(&content.page_id).await?;

        Ok(FullContent { content, page })
    }

    pub async fn find_all_by_page_id(&self, page_id: i64) -> Result<Vec<Content>, sqlx::Error> {
        self.content_repository.find_all_by_page_id(page_id).await
    }

    pub async fn create_content(
        &self,
        mut request: NewContentRequest,
    ) -> Result<Content, sqlx::Error> {
        request.name = slug::slugify(&request.name).replace("-", "_");
        self.content_repository.create_content(&request).await
    }

    pub async fn update_content(
        &self,
        request: UpdateContentRequest,
    ) -> Result<Content, sqlx::Error> {
        self.content_repository.update_content(request).await
    }

    pub async fn delete_content(&self, id: &i64) -> Result<(), sqlx::Error> {
        self.content_repository.delete_content(id).await
    }
}
