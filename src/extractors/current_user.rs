use std::{convert::Infallible, env, sync::Arc};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    AppState,
    user::{User, auth::UserClaims, repository::UserRepository, service::UserService},
};

pub struct CurrentUser(pub User);
pub struct MaybeUser(pub Option<User>);

impl FromRequestParts<Arc<AppState>> for CurrentUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|hv| hv.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing cookies"))?;

        let token = cookie_header
            .split(';')
            .map(str::trim)
            .find_map(|cookie| cookie.strip_prefix("auth_token=").map(|val| val.to_owned()))
            .ok_or((StatusCode::UNAUTHORIZED, "auth_token cookie not found"))?;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be present");
        let key = DecodingKey::from_secret(secret.as_bytes());

        let token_data = decode::<UserClaims>(&token, &key, &Validation::default())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let user_repository = UserRepository::new(&state.db);
        let user_service = UserService::new(user_repository);
        let user_id = token_data
            .claims
            .sub
            .parse::<i64>()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user id in token"))?;
        let user = user_service
            .find_user_by_id(user_id)
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Something went wrong"));

        Ok(CurrentUser(user.unwrap()))
    }
}

impl FromRequestParts<Arc<AppState>> for MaybeUser {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let cookie_header = match parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|hv| hv.to_str().ok())
        {
            Some(h) => h,
            None => return Ok(MaybeUser(None)),
        };

        let token = match cookie_header
            .split(';')
            .map(str::trim)
            .find_map(|cookie| cookie.strip_prefix("auth_token=").map(|val| val.to_owned()))
        {
            Some(t) => t,
            None => return Ok(MaybeUser(None)),
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be present");
        let key = DecodingKey::from_secret(secret.as_bytes());

        let token_data = match decode::<UserClaims>(&token, &key, &Validation::default()) {
            Ok(s) => s,
            Err(_) => return Ok(MaybeUser(None)),
        };

        let user_repository = UserRepository::new(&state.db);
        let user_service = UserService::new(user_repository);
        let user_id = match token_data.claims.sub.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Ok(MaybeUser(None)),
        };
        let user = match user_service.find_user_by_id(user_id).await {
            Ok(user) => user,
            Err(_) => return Ok(MaybeUser(None)),
        };

        Ok(MaybeUser(Some(user)))
    }
}
