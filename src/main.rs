use std::{env, sync::Arc};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sqlx::SqlitePool;
use wordford::{content::{content_repository::{ContentRepository}, content_service::ContentService}, AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables

    let db = SqlitePool::connect(env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let state = Arc::new(AppState { db });

    // Initialize the application state and routes
    let app = Router::new().route("/", get(get_content)).with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_content(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_content("home").await {
        Ok(content) => Json(content).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}