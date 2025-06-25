pub async fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
}

pub struct AuthService {
    db: sqlx::SqlitePool,
}

impl AuthService {
    pub fn new(db: sqlx::SqlitePool) -> Self {
        AuthService { db }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<bool, sqlx::Error> {
        let user = sqlx::query!("SELECT password_hash FROM users WHERE email = ?", email)
            .fetch_optional(&self.db)
            .await?;

        if let Some(user) = user {
            Ok(AuthService::verify_password(password, &user.password_hash).await)
        } else {
            Ok(false)
        }
    }

    async fn verify_password(password: &str, hashed: &str) -> bool {
        bcrypt::verify(password, hashed).unwrap_or(false)
    }
}
