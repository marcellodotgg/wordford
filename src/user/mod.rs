use serde::{Deserialize, Serialize};

pub mod repository;
pub mod routes;
pub mod service;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub avatar_url: String,
    pub role: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub given_name: String,
    pub family_name: String,
}
