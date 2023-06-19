pub mod auth;
pub mod goals;

pub fn routes() -> Vec<rocket::Route> {
    itertools::concat(vec![auth::routes(), goals::routes()])
}
