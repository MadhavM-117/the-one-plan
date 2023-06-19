pub mod auth;

pub fn routes() -> Vec<rocket::Route> {
    itertools::concat(vec![auth::routes()])
}
