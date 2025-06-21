use crate::{
    AppState, models::page::NewPageRequest, repositories::pages::PageRepository,
    services::pages::PageService,
};
use axum::{
    Form, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, put},
};
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(create_page))
        .route("/{id}", get(find_page_by_id).delete(delete_page))
        .route("/{id}/content", get(get_content_for_page))
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api/pages", api_routes())
}

pub async fn find_page_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let page_repository = PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.find_by_id(&id).await {
        Ok(Some(page)) => Json(page).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_content_for_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let page_repository = PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.get_content_for_page(&id).await {
        Ok(content) => Json(content).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_page(
    State(state): State<Arc<AppState>>,
    Form(request): Form<NewPageRequest>,
) -> impl IntoResponse {
    let page_repository = PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    let mut context = tera::Context::new();
    context.insert("app", &serde_json::json!({ "id": request.app_id }));
    context.insert("page", &request);

    let result = page_service.create_page(request).await;
    let mut error_message = |msg| {
        context.insert("error", &msg);
        Html(
            state
                .tera
                .render("apps/new_page_form.html", &context)
                .unwrap(),
        )
    };

    match result {
        Ok(page) => (
            [("HX-Redirect", format!("/apps/{}", page.app_id))],
            StatusCode::OK,
        )
            .into_response(),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            error_message("Page name already exists for this app.").into_response()
        }
        Err(sqlx::Error::Database(err)) if err.is_foreign_key_violation() => {
            error_message("The provided App ID is not valid.").into_response()
        }
        Err(_) => error_message("Something went wrong creating the page.").into_response(),
    }
}

pub async fn delete_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let page_repository = PageRepository::new(state.db.clone());
    let page_service = PageService::new(page_repository);

    match page_service.delete_page(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
