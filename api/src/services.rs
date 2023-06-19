use crate::services::auth::{AuthService, PgAuthService};
use crate::services::config::ServiceConfig;
use crate::InternalResult;
use envconfig::Envconfig;
use crate::services::goals::{GoalsService, PgGoalsService};

pub mod auth;
pub mod config;
pub mod goals;

trait DbConnection<T> {
    fn connection(&self) -> InternalResult<T>;
}

#[derive(Clone)]
pub struct Services {
    pub config: ServiceConfig,
}

impl Services {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();

        let config = ServiceConfig::init_from_env().unwrap();

        Self { config }
    }

    pub async fn auth(&self) -> InternalResult<Box<dyn AuthService>> {
        Ok(Box::from(PgAuthService::new(self.config.clone())))
    }

    pub async fn goals(&self) -> InternalResult<Box<dyn GoalsService>> {
        Ok(Box::from(PgGoalsService::new(self.config.clone())))
    }
}
