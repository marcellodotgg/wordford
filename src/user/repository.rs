use crate::user::{CreateUserRequest, User};

pub struct UserRepository {
    db: sqlx::SqlitePool,
}

impl UserRepository {
    pub fn new(db: &sqlx::SqlitePool) -> Self {
        UserRepository { db: db.clone() }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, sqlx::Error> {
        let user = sqlx::query!("SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.db)
            .await?;
        Ok(User {
            id: user.id,
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            avatar_url: user.avatar_url,
            role: user.role,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        })
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query!("SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.db)
            .await?;
        Ok(User {
            id: user.id.expect("this should not be null"),
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            avatar_url: user.avatar_url,
            role: user.role,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        })
    }

    pub async fn create_user(
        &self,
        create_user_request: &CreateUserRequest,
    ) -> Result<User, sqlx::Error> {
        let critter_num = rand::random::<u8>() % 11 + 1;
        let avatar_url = format!(
            "http://localhost:3000/assets/images/critter_{}.svg",
            critter_num
        );
        let user = sqlx::query!(
            "INSERT INTO users (email, given_name, family_name, password_hash, avatar_url) VALUES (?, ?, ?, ?, ?)",
            create_user_request.email,
            create_user_request.given_name,
            create_user_request.family_name,
            create_user_request.password,
            avatar_url
        )
        .execute(&self.db)
        .await?;

        Ok(User {
            id: user.last_insert_rowid(),
            email: create_user_request.email.to_string(),
            given_name: create_user_request.given_name.to_string(),
            family_name: create_user_request.family_name.to_string(),
            avatar_url: avatar_url,
            role: 1,
            created_at: chrono::Utc::now().to_string(),
            updated_at: chrono::Utc::now().to_string(),
        })
    }
}
