use axum::Router;
use sqlx::SqlitePool;
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use wordford::content::Content;
use wordford::content::CreateContentRequest;
use wordford::{AppState, content::content_routes, pages::page_routes};

#[derive(OpenApi)]
#[openapi(
    paths(
        content_routes::find_by_id,
        content_routes::create_content,
        content_routes::delete_content,
        page_routes::find_page_by_id,
        page_routes::create_page,
        page_routes::delete_page,
    ),
    components(schemas(Content, CreateContentRequest)),
    tags(
        (name = "Content", description = "Content management endpoints"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables

    let db = SqlitePool::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let state = Arc::new(AppState { db });

    // Initialize the application state and routes
    let app = Router::new()
        .merge(content_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
