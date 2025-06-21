use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::{AppState, repositories::pages::PageRepository, services::pages::PageService};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage_index))
}

async fn homepage_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let page_service = PageService::new(PageRepository::new(&state.db));

    match page_service.get_content_for_page_name("homepage", 1).await {
        Ok(content) => {
            let context = match tera::Context::from_serialize(content) {
                Ok(ctx) => ctx,
                Err(_) => return Html("<h1>Error preparing context</h1>".to_string()),
            };
            match state.tera.render("homepage/index.html", &context) {
                Ok(rendered) => Html(rendered),
                Err(_) => Html("Error rendering the homepage, did you you need to have <code>homepage_content</code> present in your wordford page".to_string()),
            }
        }
        Err(_) => {
            return Html("<h1>Error loading content</h1>".to_string());
        }
    }
}
