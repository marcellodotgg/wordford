use crate::{
    AppState,
    orgs::{org_repository::OrgRepository, org_service::OrgService},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api/orgs", api_routes())
}

#[utoipa::path(
    get,
    path = "/api/orgs/{id}",
    responses(
        (status = 200, description = "Get org by ID", body = String),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = String, Path, description = "Org ID")
    ),
    tag = "Orgs"
)]
pub async fn find_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let org_repository = OrgRepository::new(state.db.clone());
    let org_service = OrgService::new(org_repository);

    match org_service.find_by_id(&id).await {
        Ok(Some(page)) => Json(page).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/orgs",
    responses(
        (status = 201, description = "Org created"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Orgs"
)]
pub async fn create_org(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let org_repository = OrgRepository::new(state.db.clone());
    let org_service = OrgService::new(org_repository);

    match org_service.create_org("home").await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/orgs/{id}",
    responses(
        (status = 204, description = "Org deleted"),
        (status = 404, description = "Not found")
    ),
    params(
        ("id" = String, Path, description = "Org ID")
    ),
    tag = "Orgs"
)]
pub async fn delete_org(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let org_repository = OrgRepository::new(state.db.clone());
    let org_service = OrgService::new(org_repository);

    match org_service.delete_org(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
