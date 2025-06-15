use std::{env, sync::Arc};
use axum::Router;
use sqlx::SqlitePool;
use wordford::{content::content_routes, AppState};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables

    let db = SqlitePool::connect(env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();
    let state = Arc::new(AppState { db });

    // Initialize the application state and routes
    let app = Router::new()
        .merge(content_routes::routes())
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

