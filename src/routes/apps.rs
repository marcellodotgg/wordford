use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use std::sync::Arc;
use tera::Context;

use crate::{
    AppState,
    models::app::{App, AppSearch},
    repositories::apps::AppRepository,
    services::apps::AppService,
};

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(find_by_id))
        .route("/{id}/pages", get(find_pages_by_app_id))
        .route("/search", get(search))
}

fn html_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(app_html))
        .route("/{id}/pages/new", get(new_page_html))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api/apps", api_routes())
        .nest("/apps", html_routes())
}

pub async fn new_page_html(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Html<String> {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.find_by_id(&id).await {
        Ok(app) => {
            let context = tera::Context::from_serialize(app).unwrap();
            Html(state.tera.render("apps/new_page.html", &context).unwrap())
        }
        Err(_) => Html("".to_string()),
    }
}

pub async fn app_html(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Html<String> {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);

    match app_service.find_by_id(&id).await {
        Ok(app) => {
            let context = tera::Context::from_serialize(app).unwrap();
            Html(state.tera.render("apps/index.html", &context).unwrap())
        }
        Err(_) => Html("".to_string()),
    }
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
    Path(id): Path<i64>,
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
    path = "/api/apps/search",
    params(
        ("name" = String, Query, description = "Name to search for"),
    ),
    responses(
        (status = 200, description = "Get app by search criteria", content_type = "text/html", body = String),
        (status = 404, description = "Not found")
    ),
    tag = "Apps"
)]
pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AppSearch>,
) -> Html<String> {
    let app_repository = AppRepository::new(state.db.clone());
    let app_service = AppService::new(app_repository);
    let mut context = Context::new();

    if params.name.trim().is_empty() {
        context.insert("apps", &Vec::<App>::new());
        return Html("".to_string());
    }

    let discovered_apps = app_service.search(&params).await.unwrap_or_default();
    context.insert("apps", &discovered_apps);

    Html(
        state
            .tera
            .render("apps/_search_autofill.html", &context)
            .unwrap(),
    )
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
