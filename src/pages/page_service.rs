use crate::pages::{
    NewPageRequest, Page, PageContent, PageWithContent, page_repository::PageRepository,
};

pub struct PageService {
    page_repository: PageRepository,
}

impl PageService {
    pub fn new(page_repository: PageRepository) -> Self {
        PageService { page_repository }
    }

    pub async fn find_by_id(&self, page_id: &i64) -> Result<Option<PageWithContent>, sqlx::Error> {
        self.page_repository.find_by_id(page_id).await
    }

    pub async fn get_content_for_page(&self, page_id: &i64) -> Result<PageContent, sqlx::Error> {
        self.page_repository.get_content_for_page(page_id).await
    }

    pub async fn create_page(&self, request: NewPageRequest) -> Result<Page, sqlx::Error> {
        self.page_repository.create_page(request).await
    }

    pub async fn delete_page(&self, page_id: &str) -> Result<(), sqlx::Error> {
        self.page_repository.delete_page(page_id).await
    }
}
