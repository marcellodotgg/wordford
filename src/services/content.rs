use crate::{
    models::content::{Content, NewContentRequest},
    repositories::content::ContentRepository,
};

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Self {
        ContentService { content_repository }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Content, sqlx::Error> {
        self.content_repository.find_by_id(id).await
    }

    pub async fn create_content(
        &self,
        mut request: NewContentRequest,
    ) -> Result<Content, sqlx::Error> {
        request.name = slug::slugify(&request.name).replace("-", "_");
        self.content_repository.create_content(&request).await
    }

    pub async fn delete_content(&self, id: &i64) -> Result<(), sqlx::Error> {
        self.content_repository.delete_content(id).await
    }
}
