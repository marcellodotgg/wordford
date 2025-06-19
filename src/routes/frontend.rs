use std::sync::Arc;

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(homepage_index))
}

async fn homepage_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(
        state
            .tera
            .render("homepage/index.html", &tera::Context::new())
            .unwrap(),
    )
}
