use rocket::{get, post, patch, delete, State};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::api::guard::ApiGuard;
use crate::{ApiError, ApiResult};
use crate::models::goals::{ActionPoint, Goal};
use crate::services::goals::GoalsService;
use crate::services::Services;

pub fn routes() -> Vec<rocket::Route> {
    vec![]
}

#[get("/goals")]
pub async fn get_user_goals(
    services: &State<Services>,
    guard: Option<ApiGuard>,
) -> ApiResult<Json<Vec<Goal>>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    Ok(Json(goals.get_goals_by_user_id(user_id.as_str()).await?))
}

#[derive(Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    pub description: String,
}


#[post("/goals", data = "<req>")]
pub async fn create_goal(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    req: Json<CreateGoalRequest>,
) -> ApiResult<Json<Goal>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let req = req.into_inner();
    let mut goal = Goal::default();
    goal.user_id = user_id;
    goal.title = req.title;
    goal.description = req.description;

    Ok(Json(goals.save_goal(goal).await?))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateGoalRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[patch("/goals/<goal_id>", data = "<req>")]
pub async fn edit_goal(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    goal_id: String,
    req: Json<UpdateGoalRequest>,
) -> ApiResult<Json<Goal>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let req = req.into_inner();
    let goal = goals.get_goal_by_id(goal_id.as_str()).await?;

    if goal.is_none() {
        return Err(ApiError::NotFound);
    }

    let mut goal = goal.unwrap();

    if goal.user_id != user_id {
        return Err(ApiError::Unauthorized);
    }

    if let Some(title) = req.title {
        goal.title = title;
    }

    if let Some(description) = req.description {
        goal.description = description;
    }

    Ok(Json(goals.save_goal(goal).await?))
}

#[delete("/goals/<goal_id>")]
pub async fn delete_goal(services: &State<Services>, guard: Option<ApiGuard>, goal_id: String) -> ApiResult<()> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let goal = validate_goal_ownership(&goals, user_id.as_str(), goal_id.as_str()).await?;

    goals.delete_goal(goal).await?;

    Ok(())
}

#[get("/goals/<goal_id>/action_points")]
pub async fn get_action_points(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    goal_id: String,
) -> ApiResult<Json<Vec<ActionPoint>>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let goal = validate_goal_ownership(&goals, user_id.as_str(), goal_id.as_str()).await?;

    Ok(Json(goals.get_action_points_by_goal_id(goal.id.as_str()).await?))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActionPointRequest {
    pub action_point_type: String,
    pub description: String,
}

#[post("/goals/<goal_id>/action_points", data = "<req>")]
pub async fn create_action_point(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    goal_id: String,
    req: Json<CreateActionPointRequest>,
) -> ApiResult<Json<ActionPoint>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;
    let req = req.into_inner();

    let goal = validate_goal_ownership(&goals, user_id.as_str(), goal_id.as_str()).await?;
    let mut action_point = ActionPoint::default();
    action_point.goal_id = goal.id.clone();
    action_point.action_point_type = req.action_point_type.clone();
    action_point.description = req.description.clone();

    Ok(Json(goals.save_action_point(action_point).await?))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateActionPointRequest {
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[patch("/goals/<goal_id>/action_points/<action_point_id>", data = "<req>")]
pub async fn edit_action_point (
    services: &State<Services>,
    guard: Option<ApiGuard>,
    goal_id: String,
    action_point_id: String,
    req: Json<UpdateActionPointRequest>,
) -> ApiResult<Json<ActionPoint>> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let goal = validate_goal_ownership(&goals, user_id.as_str(), goal_id.as_str()).await?;
    let action_point = goals.get_action_point_by_id(action_point_id.as_str()).await?;

    if action_point.is_none() {
        return Err(ApiError::NotFound);
    }

    let mut action_point = action_point.unwrap();
    if action_point.goal_id.eq(goal.id.as_str()) {
        return Err(ApiError::Unauthorized);
    }

    let req = req.into_inner();
    if let Some(description) = req.description {
        action_point.description = description;
    }

    if let Some(completed) = req.completed {
        action_point.completed = completed;
    }

    Ok(Json(goals.save_action_point(action_point).await?))
}

#[delete("/goals/<goal_id>/action_points/<action_point_id>")]
pub async fn delete_action_point(
    services: &State<Services>,
    guard: Option<ApiGuard>,
    goal_id: String,
    action_point_id: String,
) -> ApiResult<()> {
    if guard.is_none() {
        return Err(ApiError::Unauthenticated);
    }

    let goals = services.goals().await?;
    let user_id = guard.unwrap().user_id;

    let goal = validate_goal_ownership(&goals, user_id.as_str(), goal_id.as_str()).await?;
    let action_point = goals.get_action_point_by_id(action_point_id.as_str()).await?;

    if action_point.is_none() {
        return Err(ApiError::NotFound);
    }

    let action_point = action_point.unwrap();
    if action_point.goal_id.eq(goal.id.as_str()) {
        return Err(ApiError::Unauthorized);
    }

    goals.delete_action_point(action_point).await?;

    Ok(())
}


async fn validate_goal_ownership(goals: &Box<dyn GoalsService>, user_id: &str, goal_id: &str) -> ApiResult<Goal> {
    let goal = goals.get_goal_by_id(goal_id).await?;

    if goal.is_none() {
        return Err(ApiError::NotFound);
    }

    let goal = goal.unwrap();

    if !goal.user_id.eq(user_id) {
        return Err(ApiError::Unauthorized);
    }

    Ok(goal)
}
