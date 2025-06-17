use crate::apps::app_repository::AppRepository;

pub struct AppService {
    app_repository: AppRepository,
}

impl AppService {
    pub fn new(app_repository: AppRepository) -> Self {
        AppService { app_repository }
    }

    pub async fn find_by_id(&self, app_id: &str) -> Result<Option<String>, sqlx::Error> {
        self.app_repository.find_by_id(app_id).await
    }

    pub async fn create_app(&self, name: &str) -> Result<String, sqlx::Error> {
        self.app_repository.create_app(name).await
    }

    pub async fn delete_app(&self, app_id: &str) -> Result<(), sqlx::Error> {
        self.app_repository.delete_app(app_id).await
    }
}
