use crate::content::Content;
use crate::pages::page_routes::find_page_by_id;
use crate::{
    AppState,
    content::{
        CreateContentRequest, content_repository::ContentRepository,
        content_service::ContentService,
    },
};
use axum::routing::put;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use sqlx::Error::{self, RowNotFound};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/content", put(create_content))
        .route("/content/{id}", get(find_page_by_id).delete(delete_content))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api_routes())
}

#[utoipa::path(
    get,
    path = "/api/page/{page_name}",
    responses(
        (status = 200, description = "Get content as JSON", body = Content),
        (status = 404, description = "Not found")
    ),
    params(
        ("page_name" = String, Path, description = "Page name")
    ),
    tag = "Content"
)]
pub async fn find_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.find_by_id(&id).await {
        Ok(content) => Json(content).into_response(),
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/page/{page_name}",
    request_body = CreateContentRequest,
    responses(
        (status = 201, description = "Content created", body = Content),
        (status = 409, description = "Conflict")
    ),
    params(
        ("page_name" = String, Path, description = "Page name")
    ),
    tag = "Content"
)]
pub async fn create_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.create_content(&id).await {
        Ok(content) => (StatusCode::CREATED, Json(content)).into_response(),
        Err(Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                (
                    StatusCode::CONFLICT,
                    "content_id is already in use for this page.",
                )
                    .into_response()
            } else {
                eprintln!("Database error: {}", db_err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/page/{page_name}/{content_id}",
    responses(
        (status = 204, description = "Content deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("page_name" = String, Path, description = "Page name"),
        ("content_id" = String, Path, description = "Content ID")
    ),
    tag = "Content"
)]
pub async fn delete_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.delete_content(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
