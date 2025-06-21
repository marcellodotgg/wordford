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

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/pages",
        Router::new()
            .route("/", put(create_page))
            .route("/{id}", get(index))
            .route("/{id}/content", get(get_content_for_page))
            .route("/{id}/content/create", get(create_content_page)),
    )
}

pub async fn create_content_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Html<String> {
    let page_repository = PageRepository::new(&state.db);
    let page_service = PageService::new(page_repository);
    let context = tera::Context::new();

    match page_service.find_by_id(&id).await {
        Ok(page) => {
            let context = tera::Context::from_serialize(page).unwrap();
            Html(
                state
                    .tera
                    .render("pages/create_content.html", &context)
                    .unwrap(),
            )
        }
        Err(sqlx::Error::RowNotFound) => {
            Html(state.tera.render("shared/404.html", &context).unwrap())
        }
        Err(_) => Html(state.tera.render("shared/500.html", &context).unwrap()),
    }
}

pub async fn index(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Html<String> {
    let page_repository = PageRepository::new(&state.db);
    let page_service = PageService::new(page_repository);

    match page_service.find_by_id(&id).await {
        Ok(page) => {
            let context = tera::Context::from_serialize(page).unwrap();
            Html(state.tera.render("pages/index.html", &context).unwrap())
        }
        Err(sqlx::Error::RowNotFound) => Html(
            state
                .tera
                .render("shared/404.html", &tera::Context::new())
                .unwrap(),
        ),
        Err(_) => Html(
            state
                .tera
                .render("shared/500.html", &tera::Context::new())
                .unwrap(),
        ),
    }
}

pub async fn get_content_for_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let page_repository = PageRepository::new(&state.db);
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
    let page_repository = PageRepository::new(&state.db);
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
