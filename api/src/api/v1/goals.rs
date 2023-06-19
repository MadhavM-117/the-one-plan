use rocket::{get, post, State};
use rocket::serde::json::Json;
use crate::api::guard::ApiGuard;
use crate::{ApiError, ApiResult};
use crate::models::goals::Goal;
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
