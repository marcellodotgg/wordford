use axum::routing::put;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

use crate::AppState;
use crate::models::content::NewContentRequest;
use crate::repositories::content::ContentRepository;
use crate::services::content::ContentService;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(create_content))
        .route("/{id}", get(find_by_id).delete(delete_content))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api/content", api_routes())
}

pub async fn find_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.find_by_id(&id).await {
        Ok(content) => Json(content).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_content(
    State(state): State<Arc<AppState>>,
    Json(request): Json<NewContentRequest>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.create_content(request).await {
        Ok(content) => (StatusCode::CREATED, Json(content)).into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => (
            StatusCode::CONFLICT,
            "that name is already in use for this page.",
        )
            .into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.is_foreign_key_violation() => (
            StatusCode::CONFLICT,
            "the page_id does not exist or is invalid.",
        )
            .into_response(),
        Err(sqlx::Error::Database(db_err)) => {
            eprintln!("Database error: {}", db_err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.delete_content(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
