use crate::{
    AppState,
    user::{
        CreateUserRequest, SignInRequest, auth::AuthService, repository::UserRepository,
        service::UserService,
    },
};
use axum::{
    Form, Router,
    extract::State,
    http::{HeaderValue, header::SET_COOKIE},
    response::{Html, IntoResponse},
    routing::{get, put},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use std::sync::Arc;
use time::Duration;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/users", Router::new().route("/", put(create_user)))
        .route("/signup", get(signup_html))
        .route("/signin", get(signin_html).put(signin))
}

pub async fn signin_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Html(
        state
            .tera
            .render("auth/signin.html", &tera::Context::new())
            .unwrap(),
    )
}

pub async fn signin(
    State(state): State<Arc<AppState>>,
    Form(request): Form<SignInRequest>,
) -> impl IntoResponse {
    let auth_service = AuthService::new(state.db.clone());
    let mut context = tera::Context::from_serialize(&request).unwrap();

    let template = "auth/signin_form.html";
    match auth_service.login(&request.email, &request.password).await {
        Ok(token) => match token {
            Some(token) => {
                let cookie = Cookie::build(("auth_token", token))
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Lax)
                    .secure(true)
                    .max_age(Duration::days(365))
                    .build();

                context.insert("success", "Login successful!");
                let body = state.tera.render(template, &context).unwrap();

                (
                    [(
                        SET_COOKIE,
                        HeaderValue::from_str(&cookie.to_string())
                            .expect("failed to convert cookie to string"),
                    )],
                    Html(body),
                )
                    .into_response()
            }
            None => {
                context.insert("error", "Invalid email or password. Please try again.");
                state
                    .tera
                    .render(template, &context)
                    .unwrap()
                    .into_response()
            }
        },
        Err(_) => {
            context.insert("error", "An unexpected error occurred. Please try again.");
            state
                .tera
                .render(template, &context)
                .unwrap()
                .into_response()
        }
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(request): Form<CreateUserRequest>,
) -> impl IntoResponse {
    let user_service = UserService::new(UserRepository::new(&state.db));
    let mut context = tera::Context::new();

    let template = "user/create_user_form.html";
    let mut request = request.clone();
    let html = match user_service.create_user(&mut request).await {
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
