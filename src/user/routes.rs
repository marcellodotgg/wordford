use crate::{
    AppState,
    user::{CreateUserRequest, repository::UserRepository, service::UserService},
};
use axum::{
    Form, Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::put,
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/users", Router::new().route("/", put(create_user)))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(request): Form<CreateUserRequest>,
) -> impl IntoResponse {
    let user_repository = UserRepository::new(&state.db);
    let user_service = UserService::new(user_repository);

    match user_service.create_user(&request).await {
        Ok(_) => Html("nice").into_response(),
        Err(err) => Html(err.to_string()).into_response(),
    }
}
