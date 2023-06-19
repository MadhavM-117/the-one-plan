use diesel::{Connection, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use dyn_clone::{clone_trait_object, DynClone};
use crate::InternalResult;
use crate::models::goals::{ActionPoint, Goal};
use crate::schema::{goals, action_points};
use crate::services::config::ServiceConfig;
use crate::services::DbConnection;

/// Goals Service is meant to help with the following:
/// - Getting Goals
/// - Creating / Editing Goals
/// - Deleting Goals
/// - Getting ActionPoints
/// - Creating / Editing ActionPoints
/// - Deleting ActionPoints
#[async_trait]
pub trait GoalsService: Sync + Send + DynClone {
    async fn get_goals_by_user_id(&self, user_id: &str) -> InternalResult<Vec<Goal>>;

    async fn get_goal_by_id(&self, goal_id: &str) -> InternalResult<Option<Goal>>;

    async fn save_goal(&self, goal: Goal) -> InternalResult<Goal>;

    async fn delete_goal(&self, goal: Goal) -> InternalResult<()>;

    async fn get_action_points_by_goal_id(&self, goal_id: &str) -> InternalResult<Vec<ActionPoint>>;

    async fn get_action_point_by_id(&self, action_point_id: &str) -> InternalResult<Option<ActionPoint>>;

    async fn save_action_point(&self, action_point: ActionPoint) -> InternalResult<ActionPoint>;

    async fn delete_action_point(&self, action_point: ActionPoint) -> InternalResult<()>;
}

clone_trait_object!(GoalsService);

#[derive(Clone)]
pub struct PgGoalsService {
    pub config: ServiceConfig,
}

impl PgGoalsService {
    pub fn new(config: ServiceConfig) -> Self {
        Self { config }
    }
}

impl DbConnection<PgConnection> for PgGoalsService {
    fn connection(&self) -> InternalResult<PgConnection> {
        Ok(PgConnection::establish(self.config.get_db_url())?)
    }
}

#[async_trait]
impl GoalsService for PgGoalsService {
    async fn get_goals_by_user_id(&self, user_id: &str) -> InternalResult<Vec<Goal>> {
        Ok(goals::table
            .filter(goal::user_id.eq(user_id))
            .get_results(&mut self.connection()?)?)
    }

    async fn get_goal_by_id(&self, goal_id: &str) -> InternalResult<Option<Goal>> {
        Ok(goals::table.filter(goal::id.eq(goal_id))
            .get_result(&mut self.connection()?)
            .optional()?)
    }

    async fn save_goal(&self, goal: Goal) -> InternalResult<Goal> {
        Ok(
            diesel::insert_into(goals::table)
                .values(&goal).on_conflict(goal::id)
                .do_update()
                .set(&goal)
                .get_result(&mut self.connection()?)?
        )
    }

    async fn delete_goal(&self, goal: Goal) -> InternalResult<()> {
        Ok(diesel::delete(goals::table.filter(goal::id.eq(goal.id)))
            .execute(&mut self.connection()?).map(|_| ())?)
    }

    async fn get_action_points_by_goal_id(&self, goal_id: &str) -> InternalResult<Vec<ActionPoint>> {
        Ok(action_points::table
            .filter(action_point::goal_id.eq(goal_id))
            .get_results(&mut self.connection()?)?)
    }

    async fn get_action_point_by_id(&self, action_point_id: &str) -> InternalResult<Option<ActionPoint>> {
        Ok(action_points::table.filter(action_point::id.eq(action_point_id))
            .get_result(&mut self.connection()?)
            .optional()?)
    }

    async fn save_action_point(&self, action_point: ActionPoint) -> InternalResult<ActionPoint> {
        Ok(
            diesel::insert_into(action_points::table)
                .values(&action_point).on_conflict(action_point::id)
                .do_update()
                .set(&action_point)
                .get_result(&mut self.connection()?)?
        )
    }

    async fn delete_action_point(&self, action_point: ActionPoint) -> InternalResult<()> {
        Ok(diesel::delete(action_points::table.filter(action_point::id.eq(action_point.id)))
            .execute(&mut self.connection()?).map(|_| ())?)
    }
}
