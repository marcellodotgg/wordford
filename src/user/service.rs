use crate::user::{CreateUserRequest, User, repository::UserRepository};

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        self.repository.find_by_email(email).await
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<User, sqlx::Error> {
        self.repository.create_user(&request).await
    }
}
