use crate::content::content_repository::ContentRepository;

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Self {
        ContentService { content_repository }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        self.content_repository.find_by_id(id).await
    }

    pub async fn create_content(&self, page_id: &str) -> Result<String, sqlx::Error> {
        self.content_repository
            .create_content(page_id, "sample")
            .await
    }

    pub async fn delete_content(&self, content_id: &str) -> Result<(), sqlx::Error> {
        self.content_repository.delete_content(content_id).await
    }
}
