use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sqlx::Error::RowNotFound;
use crate::{content::{content_repository::ContentRepository, content_service::ContentService}, AppState};

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sitemap", get(get_sitemap_json))
        .route("/page/{page_name}", get(get_content_json))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api_routes())
}

async fn get_content_json(State(state): State<Arc<AppState>>, Path(page_name): Path<String>) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_content(&page_name).await {
        Ok(content) => Json(content).into_response(),
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_sitemap_json(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_sitemap().await {
        Ok(sitemap) => Json(sitemap).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
