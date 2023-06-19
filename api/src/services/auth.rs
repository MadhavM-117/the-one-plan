use crate::models::auth::{AuthSession, User};
use crate::schema::{auth_sessions, users};
use crate::services::config::ServiceConfig;
use crate::services::DbConnection;
use crate::InternalResult;
use async_trait::async_trait;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};
use dyn_clone::{clone_trait_object, DynClone};

/// AuthService is meant to help with anything auth related, such as:
/// - Getting Users
/// - Creating / Editing Users
/// - Creating / Editing AuthSessions
#[async_trait]
pub trait AuthService: Sync + Send + DynClone {
    async fn get_user_by_email(&self, email_id: &str) -> InternalResult<Option<User>>;

    async fn get_user_by_id(&self, user_id: &str) -> InternalResult<Option<User>>;

    async fn save_user(&self, user: User) -> InternalResult<User>;

    async fn get_auth_session_by_id(
        &self,
        auth_session_id: &str,
    ) -> InternalResult<Option<AuthSession>>;

    async fn save_auth_session(&self, auth_session: AuthSession) -> InternalResult<AuthSession>;

    async fn delete_auth_session(&self, auth_session: AuthSession) -> InternalResult<()>;
}

clone_trait_object!(AuthService);

#[derive(Clone)]
pub struct PgAuthService {
    pub config: ServiceConfig,
}

impl PgAuthService {
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
}

impl DbConnection<PgConnection> for PgAuthService {
    fn connection(&self) -> InternalResult<PgConnection> {
        Ok(PgConnection::establish(self.config.get_db_url())?)
    }
}

#[async_trait]
impl AuthService for PgAuthService {
    async fn get_user_by_email(&self, email_id: &str) -> InternalResult<Option<User>> {
        Ok(users::table
            .filter(users::email.eq(email_id))
            .get_result(&mut self.connection()?)
            .optional()?)
    }

    async fn get_user_by_id(&self, user_id: &str) -> InternalResult<Option<User>> {
        Ok(users::table
            .find(user_id)
            .get_result(&mut self.connection()?)
            .optional()?)
    }

    async fn save_user(&self, user: User) -> InternalResult<User> {
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .on_conflict(users::id)
            .do_update()
            .set(&user)
            .get_result(&mut self.connection()?)?)
    }

    async fn get_auth_session_by_id(
        &self,
        auth_session_id: &str,
    ) -> InternalResult<Option<AuthSession>> {
        Ok(auth_sessions::table
            .find(auth_session_id)
            .get_result(&mut self.connection()?)
            .optional()?)
    }

    async fn save_auth_session(&self, auth_session: AuthSession) -> InternalResult<AuthSession> {
        Ok(diesel::insert_into(auth_sessions::table)
            .values(&auth_session)
            .on_conflict(auth_sessions::id)
            .do_update()
            .set(&auth_session)
            .get_result(&mut self.connection()?)?)
    }

    async fn delete_auth_session(&self, auth_session: AuthSession) -> InternalResult<()> {
        Ok(
            diesel::delete(auth_sessions::table.find(auth_session.id.as_str()))
                .execute(&mut self.connection()?)
                .map(|_| ())?,
        )
    }
}
