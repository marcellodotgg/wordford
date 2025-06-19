use crate::{
    AppState,
    api::apps::{app_repository::AppRepository, app_service::AppService},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(find_by_id))
        .route("/{id}/pages", get(find_pages_by_app_id))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api/apps", api_routes())
}

#[utoipa::path(
    get,
    path = "/api/apps/{id}",
    responses(
        (status = 200, description = "Get app by ID", body = String),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = i64, Path, description = "App ID")
    ),
    tag = "Apps"
)]
pub async fn find_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.find_by_id(&id).await {
        Ok(app) => Json(app).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/apps/{id}/pages",
    responses(
        (status = 200, description = "Get pages by app ID", body = String),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = i64, Path, description = "App ID")
    ),
    tag = "Apps"
)]
pub async fn find_pages_by_app_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.find_pages_by_app_id(&id).await {
        Ok(pages) => Json(pages).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/apps",
    responses(
        (status = 201, description = "Apps created"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Apps"
)]
pub async fn create_app(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.create_app("home").await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/apps/{id}",
    responses(
        (status = 204, description = "App deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = i64, Path, description = "App ID")
    ),
    tag = "Apps"
)]
pub async fn delete_app(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.delete_app(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
