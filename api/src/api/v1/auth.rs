use crate::api::guard::{ApiGuard, LoginGuard};
use crate::models::auth::{AuthSession, User, UserData};
use crate::services::Services;
use crate::utils::{create_password_hash, new_id, now, AUTH_COOKIE_NAME};
use crate::{ApiError, ApiResult};
use chrono::Duration;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::{get, post, routes, State};
use serde::{Deserialize, Serialize};

pub fn routes() -> Vec<rocket::Route> {
    routes![whoami, signup, login, logout]
}

#[get("/auth/whoami")]
pub async fn whoami(
    services: &State<Services>,
    guard: Option<ApiGuard>,
) -> ApiResult<Json<UserData>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let auth = services.auth().await?;

    let user_id = guard.unwrap().user_id;
    if let Some(user) = auth.get_user_by_id(user_id.as_str()).await? {
        Ok(Json(user.into()))
    } else {
        Err(ApiError::NotFound)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    name: String,
    email_id: String,
    password: String,
}

#[post("/auth/signup", data = "<req>")]
pub async fn signup(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    cookies: &CookieJar<'_>,
    req: Json<SignupRequest>,
) -> ApiResult<Json<UserData>> {
    if guard.is_some() {
        return Err(ApiError::AlreadyLoggedIn);
    }

    let auth = services.auth().await?;

    let req = req.into_inner();

    let user = User {
        id: new_id(),
        name: req.name.clone(),
        email: req.email_id.clone(),
        password: create_password_hash(req.password.as_str())?,
        phone_number: None,
    };

    let user = auth.save_user(user).await?;

    let auth_session = AuthSession {
        id: new_id(),
        user_id: user.id.clone(),
        created_at: now(),
        expires_at: now() + Duration::days(7),
    };

    let auth_session = auth.save_auth_session(auth_session).await?;
    cookies.add_private(Cookie::new(AUTH_COOKIE_NAME, auth_session.id));

    Ok(Json(user.into()))
}

#[get("/auth/login")]
pub async fn login(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    login: Option<LoginGuard>,
    cookies: &CookieJar<'_>,
) -> ApiResult<Json<UserData>> {
    if guard.is_some() {
        return Err(ApiError::AlreadyLoggedIn);
    }

    if login.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let auth = services.auth().await?;

    let user = login.unwrap().user;

    let auth_session = AuthSession {
        id: new_id(),
        user_id: user.id.clone(),
        created_at: now(),
        expires_at: now() + Duration::days(7),
    };

    let auth_session = auth.save_auth_session(auth_session).await?;
    cookies.add_private(Cookie::new(AUTH_COOKIE_NAME, auth_session.id));

    Ok(Json(user.into()))
}

#[get("/auth/logout")]
pub async fn logout(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    cookies: &CookieJar<'_>,
) -> ApiResult<()> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let auth = services.auth().await?;
    let auth_session = guard.unwrap().auth_session;

    cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
    auth.delete_auth_session(auth_session).await?;

    Ok(())
}
