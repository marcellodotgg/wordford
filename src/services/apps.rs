use crate::{
    models::{
        app::{App, AppSearch, AppWithPages, CreateAppForm},
        page::Page,
    },
    repositories::apps::AppRepository,
};

pub struct AppService {
    app_repository: AppRepository,
}

impl AppService {
    pub fn new(app_repository: AppRepository) -> Self {
        AppService { app_repository }
    }

    pub async fn find_by_id(&self, app_id: &i64) -> Result<AppWithPages, sqlx::Error> {
        self.app_repository.find_by_id(app_id).await
    }

    pub async fn search(&self, params: &AppSearch) -> Result<Vec<App>, sqlx::Error> {
        self.app_repository.search(&params.name).await
    }

    pub async fn find_pages_by_app_id(&self, app_id: &str) -> Result<Vec<Page>, sqlx::Error> {
        self.app_repository.find_pages_by_app_id(app_id).await
    }

    pub async fn create_app(&self, request: CreateAppForm) -> Result<App, sqlx::Error> {
        self.app_repository.create_app(request).await
    }

    pub async fn delete_app(&self, app_id: &str) -> Result<(), sqlx::Error> {
        self.app_repository.delete_app(app_id).await
    }
}
