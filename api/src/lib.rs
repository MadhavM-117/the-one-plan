#[macro_use]
extern crate diesel;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use thiserror::Error;

pub mod api;
pub mod models;
pub mod schema;
pub mod services;
pub mod utils;

/// ApiResult is meant to be used for API functions.
/// All errors associated with this type, will be mapped to an appropriate HTTP status code,
/// and error message.
///
/// These errors will be exposed to the end-user, and so, special care must be taken with content
/// and the amount of information revealed.
pub type ApiResult<T> = anyhow::Result<T, ApiError>;

/// A result meant to be used for internal logic.
/// Any errors will be mapped to a 500 error code, unless explicitly converted to a
/// ApiError.
///
/// This is ideal for internal services & logic, where the end-user doesn't need to know
/// what happened. If the user does need to be informed, then it must be mapped to an ApiError.
pub type InternalResult<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("You must be logged in to access this resource.")]
    Unauthenticated,
    #[error("You are not authorized to access this resource.")]
    Unauthorized,
    #[error("You are already logged in.")]
    AlreadyLoggedIn,
    #[error("We couldn't find the resource you were looking for.")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &Request<'_>) -> rocket::response::Result<'o> {
        let code = match &self {
            Self::Unauthenticated => 401,
            Self::Unauthorized => 403,
            Self::AlreadyLoggedIn => 409,
            Self::NotFound => 400,
            Self::BadRequest(_) => 400,
            Self::Unknown(_) => 500,
        };

        let error_message = serde_json::to_string(&ErrorResponse {
            message: format!("{}", &self),
        })
        .unwrap_or_else(|e| {
            format!(
                "{{ \"message\": \"Failed to parse error details: {:?} \"}}",
                e
            )
        })
        .into_bytes();

        Response::build()
            .sized_body(error_message.len(), Cursor::new(error_message))
            .header(ContentType::JSON)
            .status(Status::new(code))
            .ok()
    }
}
