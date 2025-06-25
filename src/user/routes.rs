use crate::{
    AppState,
    user::{CreateUserRequest, repository::UserRepository, service::UserService},
};
use axum::{
    Form, Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, put},
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/users", Router::new().route("/", put(create_user)))
        .route("/signup", get(signup_html))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(request): Form<CreateUserRequest>,
) -> impl IntoResponse {
    let user_service = UserService::new(UserRepository::new(&state.db));
    let mut context = tera::Context::new();

    let template = "user/create_user_form.html";
    let html = match user_service.create_user(&request).await {
        Ok(_) => {
            context.insert("success", "Congratulations! You may now sign in.");
            state.tera.render(template, &context)
        }
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            context = tera::Context::from_serialize(request).unwrap();
            context.insert("error", "Email already exists. Please try again.");
            state.tera.render(template, &context)
        }
        Err(_) => {
            context = tera::Context::from_serialize(request).unwrap();
            context.insert("error", "An unexpected error occurred. Please try again.");
            state.tera.render(template, &context)
        }
    }
    .unwrap();

    Html(html)
}

pub async fn signup_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(
        state
            .tera
            .render("user/signup.html", &tera::Context::new())
            .unwrap(),
    )
}
