use crate::AppState;
use axum::Router;
use std::sync::Arc;

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/api", api_routes())
}
