use crate::pages::page_repository::PageRepository;

pub struct PageService {
    page_repository: PageRepository,
}

impl PageService {
    pub fn new(page_repository: PageRepository) -> Self {
        PageService { page_repository }
    }
}
