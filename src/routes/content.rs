use axum::{
    Form, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use std::sync::Arc;

use crate::models::content::NewContentRequest;
use crate::repositories::content::ContentRepository;
use crate::services::content::ContentService;
use crate::{AppState, models::content::FindContentByPageIdParams};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/content",
        Router::new()
            .route("/", get(find_all_by_page_id).put(create_content))
            .route("/{id}", get(find_by_id).delete(delete_content)),
    )
}

pub async fn find_all_by_page_id(
    State(state): State<Arc<AppState>>,
    Query(params): Query<FindContentByPageIdParams>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    let contents = match content_service.find_all_by_page_id(params.page_id).await {
        Ok(contents) => contents,
        Err(sqlx::Error::RowNotFound) => Vec::new(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let context =
        tera::Context::from_serialize(serde_json::json!({ "content": contents })).unwrap();
    Html(
        state
            .tera
            .render("content/list_view.html", &context)
            .unwrap(),
    )
    .into_response()
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
    Form(request): Form<NewContentRequest>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    let page_id = request.page_id;
    let name = request.name.clone();
    let body = request.body.clone();
    let error_message = |msg: &str| {
        let mut context = tera::Context::new();
        context.insert("error", &msg);
        context.insert("page_id", &page_id);
        context.insert("name", &name);
        context.insert("body", &body);
        Html(state.tera.render("content/form.html", &context).unwrap()).into_response()
    };

    match content_service.create_content(request).await {
        Ok(content) => {
            let mut context = tera::Context::new();
            context.insert("page_id", &content.page_id);
            context.insert(
                "success",
                &"Created the content, you can add more content below, or go back to the page."
                    .to_string(),
            );
            Html(state.tera.render("content/form.html", &context).unwrap()).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            error_message(&"Content with this name already exists.").into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_foreign_key_violation() => {
            error_message(&"The page you're adding to does not appear to exist.")
                .into_response()
                .into_response()
        }
        Err(_) => error_message(&"Something happened when saving your content").into_response(),
    }
}

pub async fn delete_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let content_repository = ContentRepository::new(state.db.clone());
    let content_service = ContentService::new(content_repository);

    match content_service.delete_content(&id).await {
        Ok(_) => Html("").into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
