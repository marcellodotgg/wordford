use axum::{
    Form, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, put},
};
use std::sync::Arc;
use tera::Context;

use crate::{
    AppState,
    models::app::{App, AppSearch, CreateAppForm},
    repositories::apps::AppRepository,
    services::apps::AppService,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest(
        "/apps",
        Router::new()
            .route("/", put(create_app))
            .route("/new", get(create_app_html))
            .route("/{id}", get(index))
            .route("/{id}/pages/new", get(create_new_page))
            .route("/search", get(search_results)),
    )
}

pub async fn index(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Html<String> {
    let app_repository = AppRepository::new(&state.db);
    let app_service = AppService::new(app_repository);

    match app_service.find_by_id(&id).await {
        Ok(app) => {
            let context = tera::Context::from_serialize(app).unwrap();
            Html(state.tera.render("apps/index.html", &context).unwrap())
        }
        Err(_) => Html("".to_string()),
    }
}

pub async fn create_app(
    State(state): State<Arc<AppState>>,
    Form(request): Form<CreateAppForm>,
) -> impl IntoResponse {
    let app_repository = AppRepository::new(&state.db);
    let app_service = AppService::new(app_repository);

    match app_service.create_app(request).await {
        Ok(app) => [("HX-Redirect", format!("/apps/{}", app.id))].into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_app_html(State(state): State<Arc<AppState>>) -> Html<String> {
    let context = Context::new();
    Html(state.tera.render("apps/new.html", &context).unwrap())
}

pub async fn create_new_page(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Html<String> {
    let app_repository = AppRepository::new(&state.db);
    let app_service = AppService::new(app_repository);

    match app_service.find_by_id(&id).await {
        Ok(app) => {
            let context = tera::Context::from_serialize(app).unwrap();
            Html(state.tera.render("apps/new_page.html", &context).unwrap())
        }
        Err(_) => Html("".to_string()),
    }
}

pub async fn search_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AppSearch>,
) -> Html<String> {
    let app_repository = AppRepository::new(&state.db);
    let app_service = AppService::new(app_repository);
    let mut context = Context::new();

    // If the search name is empty, return an empty list of apps
    if params.name.trim().is_empty() {
        context.insert("apps", &Vec::<App>::new());
        return Html("".to_string());
    }

    let discovered_apps = app_service.search(&params).await.unwrap_or_default();
    context.insert("apps", &discovered_apps);

    Html(
        state
            .tera
            .render("apps/search_results.html", &context)
            .unwrap(),
    )
}

pub async fn delete_app(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let app_repository = AppRepository::new(&state.db);
    let app_service = AppService::new(app_repository);

    match app_service.delete_app(&id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
