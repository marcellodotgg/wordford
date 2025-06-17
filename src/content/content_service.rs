use std::collections::HashMap;

use crate::content::{Content, CreateContentRequest, content_repository::ContentRepository};

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Self {
        ContentService { content_repository }
    }

    pub async fn get_content(
        &self,
        page_name: &str,
    ) -> Result<HashMap<String, String>, sqlx::Error> {
        self.content_repository.get_content(page_name).await
    }

    pub async fn get_sitemap(&self) -> Result<Vec<String>, sqlx::Error> {
        self.content_repository.get_sitemap().await
    }

    pub async fn create_content(
        &self,
        page_name: &str,
        request: CreateContentRequest,
    ) -> Result<Content, sqlx::Error> {
        self.content_repository
            .create_content(&page_name, request)
            .await
    }

    pub async fn delete_content(
        &self,
        page_name: &str,
        content_id: &str,
    ) -> Result<(), sqlx::Error> {
        self.content_repository
            .delete_content(page_name, content_id)
            .await
    }
}
