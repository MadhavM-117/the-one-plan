use chrono::offset::Utc;
use http_auth_basic::Credentials;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::models::auth::{AuthSession, User};
use crate::services::Services;
use crate::utils::{AUTH_COOKIE_NAME};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiGuard {
    pub user_id: String,
    pub auth_session: AuthSession,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = request.cookies().get_private(AUTH_COOKIE_NAME);
        let services = request
            .guard::<&rocket::State<Services>>()
            .await
            .expect("Error obtaining services state.");
        let auth = services.auth().await;

        if auth.is_err() {
            return Outcome::Forward(());
        }

        let auth = auth.unwrap();

        if cookie.is_none() {
            return Outcome::Forward(());
        }

        let cookie = cookie.unwrap();
        let session = auth
            .get_auth_session_by_id(cookie.value())
            .await
            .unwrap_or(None);

        if session.is_none() {
            return Outcome::Forward(());
        }

        let session = session.unwrap();
        let now = Utc::now().naive_utc();

        if now < session.expires_at.clone() {
            return Outcome::Success(ApiGuard {
                user_id: session.user_id.clone(),
                auth_session: session.clone(),
            });
        }

        Outcome::Forward(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginGuard {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        let services = request
            .guard::<&rocket::State<Services>>()
            .await
            .expect("Error obtaining services state.");
        let auth = services.auth().await;

        if auth.is_err() {
            return Outcome::Forward(());
        }

        let auth = auth.unwrap();

        let auth_header = match headers.get_one("authorization") {
            Some(a) => String::from(a),
            None => return Outcome::Forward(()),
        };

        let creds = match Credentials::from_header(auth_header).ok() {
            Some(c) => c,
            None => return Outcome::Forward(()),
        };

        let user = match auth.get_user_by_email(creds.user_id.as_str()).await.ok() {
            Some(u) => {
                if let Some(u) = u {
                    u
                } else {
                    return Outcome::Forward(());
                }
            }
            None => return Outcome::Forward(()),
        };

        let valid = bcrypt::verify(creds.password, &user.password).unwrap_or(false);

        if valid {
            return Outcome::Success(LoginGuard { user });
        }

        Outcome::Forward(())
    }
}
