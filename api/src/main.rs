use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use vignette::api::{self, health};
use vignette::services::Services;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let services = Services::new().await;
    let mut connection = PgConnection::establish(services.config.get_db_url()).unwrap();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Couldn't run migrations");

    let _rocket = rocket::build()
        .mount("/", rocket::routes![health])
        .mount("/api/v1/", api::v1::routes())
        .manage(Services::new().await)
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
