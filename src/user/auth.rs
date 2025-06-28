use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

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

    pub async fn login(&self, email: &str, password: &str) -> Result<Option<String>, sqlx::Error> {
        let user = sqlx::query!(
            "SELECT id, email, password_hash FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.db)
        .await?;

        if let Some(user) = user {
            if AuthService::verify_password(password, &user.password_hash).await {
                let expiration = Utc::now()
                    .checked_add_signed(Duration::days(1))
                    .expect("valid timestamp")
                    .timestamp() as usize;
                let claims = UserClaims {
                    sub: user.id.unwrap().to_string(),
                    email: user.email.to_string(),
                    exp: expiration,
                };
                let header = Header::new(Algorithm::HS256);
                let encoding_key =
                    EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());

                let token = encode(&header, &claims, &encoding_key)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

                Ok(Some(token))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn verify_password(password: &str, hashed: &str) -> bool {
        bcrypt::verify(password, hashed).unwrap_or(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}
