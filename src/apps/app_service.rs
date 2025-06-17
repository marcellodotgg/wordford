use crate::apps::app_repository::AppRepository;

pub struct AppService {
    app_repository: AppRepository,
}

impl AppService {
    pub fn new(app_repository: AppRepository) -> Self {
        AppService { app_repository }
    }
}
