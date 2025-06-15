use crate::content::{content_repository::ContentRepository, Content};

pub struct ContentService {
    content_repository: ContentRepository,
}

impl ContentService {
    pub fn new(content_repository: ContentRepository) -> Self {
        ContentService { content_repository }
    }

    pub async fn get_content(&self, page_name: &str) -> Result<Vec<Content>, sqlx::Error> {
        self.content_repository.get_content(page_name).await
    }
}