use crate::pages::page_repository::PageRepository;

pub struct PageService {
    page_repository: PageRepository,
}

impl PageService {
    pub fn new(page_repository: PageRepository) -> Self {
        PageService { page_repository }
    }

    pub async fn find_by_id(&self, page_id: &str) -> Result<Option<String>, sqlx::Error> {
        self.page_repository.find_by_id(page_id).await
    }

    pub async fn create_page(&self, app_id: &str, name: &str) -> Result<String, sqlx::Error> {
        self.page_repository.create_page(app_id, name).await
    }

    pub async fn delete_page(&self, page_id: &str) -> Result<(), sqlx::Error> {
        self.page_repository.delete_page(page_id).await
    }
}
