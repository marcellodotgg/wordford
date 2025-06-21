use axum::{
    Router,
    http::{HeaderValue, header::CACHE_CONTROL},
};
use sqlx::SqlitePool;
use std::{env, sync::Arc};
use tera::Tera;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use wordford::{
    AppState,
    routes::{self, homepage},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables

    let tera = Arc::new(Tera::new("templates/**/*").unwrap());
    let serve_static = Router::new()
        .nest_service("/assets", ServeDir::new("public"))
        .layer(SetResponseHeaderLayer::if_not_present(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000"),
        ));

    let db = SqlitePool::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let state = Arc::new(AppState { db, tera });

    // Initialize the application state and routes
    let app = Router::new()
        .merge(serve_static)
        .merge(homepage::routes())
        .merge(routes::content::routes())
        .merge(routes::pages::routes())
        .merge(routes::apps::routes())
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
