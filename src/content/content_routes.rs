use crate::content::Content;
use crate::{
    AppState,
    content::{
        CreateContentRequest, content_repository::ContentRepository,
        content_service::ContentService,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use sqlx::Error::{self, RowNotFound};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sitemap", get(get_sitemap_json))
        .route(
            "/page/{page_name}",
            get(get_content_json).put(create_content),
        )
        .route(
            "/page/{page_name}/{content_id}",
            get(get_content_html).delete(delete_content),
        )
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
pub async fn get_content_json(
    State(state): State<Arc<AppState>>,
    Path(page_name): Path<String>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_content(&page_name).await {
        Ok(content) => Json(content).into_response(),
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/page/{page_name}/{content_id}",
    responses(
        (status = 200, description = "Get content as HTML", body = String),
        (status = 404, description = "Not found")
    ),
    params(
        ("page_name" = String, Path, description = "Page name"),
        ("content_id" = String, Path, description = "Content ID")
    ),
    tag = "Content"
)]
pub async fn get_content_html(
    State(state): State<Arc<AppState>>,
    Path((page_name, content_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_content(&page_name).await {
        Ok(content) => match content.get(&content_id) {
            Some(content_data) => Html(content_data.clone()).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/sitemap",
    responses(
        (status = 200, description = "Get sitemap as JSON", body = [String])
    ),
    tag = "Content"
)]
pub async fn get_sitemap_json(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.get_sitemap().await {
        Ok(sitemap) => Json(sitemap).into_response(),
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
    Path(page_name): Path<String>,
    Json(request): Json<CreateContentRequest>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.create_content(&page_name, request).await {
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
    Path((page_name, content_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service
        .delete_content(&page_name, &content_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
