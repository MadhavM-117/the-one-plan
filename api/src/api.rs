use crate::ApiResult;
use rocket::get;

pub mod guard;
pub mod v1;

#[get("/")]
pub fn health() -> ApiResult<()> {
    Ok(())
}
