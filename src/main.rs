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
    api::{apps::app_routes, content::content_routes, pages::page_routes},
    routes::frontend,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        content_routes::find_by_id,
        content_routes::create_content,
        content_routes::delete_content,
        page_routes::find_page_by_id,
        page_routes::get_content_for_page,
        page_routes::create_page,
        page_routes::delete_page,
        app_routes::find_by_id,
        app_routes::find_pages_by_app_id,
        app_routes::create_app,
        app_routes::delete_app,
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
        .merge(frontend::routes())
        .merge(content_routes::routes())
        .merge(page_routes::routes())
        .merge(app_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
