use axum::{
    Router,
    http::{HeaderValue, header::CACHE_CONTROL},
};
use sqlx::SqlitePool;
use std::{env, sync::Arc};
use tera::Tera;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use wordford::{
    AppState,
    routes::{self, homepage},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::content::find_by_id,
        routes::content::create_content,
        routes::content::delete_content,
        routes::pages::find_page_by_id,
        routes::pages::get_content_for_page,
        routes::pages::create_page,
        routes::pages::delete_page,
        routes::apps::find_by_id,
        routes::apps::find_pages_by_app_id,
        routes::apps::create_app,
        routes::apps::delete_app,
    ),
    tags(
        (name = "Content Management", description = "Content management endpoints"),
    )
)]
struct ApiDoc;

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
