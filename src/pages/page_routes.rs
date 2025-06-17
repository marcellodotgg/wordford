use crate::{
    AppState,
    pages::{page_repository, page_service::PageService},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/pages", put(create_page))
        .route("/pages/{id}", get(find_page_by_id).delete(delete_page))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api_routes())
}

#[utoipa::path(
    get,
    path = "/api/pages/{id}",
    responses(
        (status = 200, description = "Get page by ID", body = String),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = String, Path, description = "Page ID")
    ),
    tag = "Pages"
)]
pub async fn find_page_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let page_repository = page_repository::PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.find_by_id(&id).await {
        Ok(Some(page)) => Json(page).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/pages",
    responses(
        (status = 201, description = "Page created"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Pages"
)]
pub async fn create_page(
    State(state): State<Arc<AppState>>,
    Path(app_id): Path<String>,
) -> impl IntoResponse {
    let page_repository = page_repository::PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.create_page(&app_id, "home").await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/pages/{id}",
    responses(
        (status = 204, description = "Page deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = String, Path, description = "Page ID")
    ),
    tag = "Pages"
)]
pub async fn delete_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let page_repository = page_repository::PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.delete_page(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
