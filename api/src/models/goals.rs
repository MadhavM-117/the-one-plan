use chrono::NaiveDateTime;
use crate::schema::{goals, action_points};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = goals)]
pub struct Goal {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = action_points)]
pub struct ActionPoint {
    pub id: String,
    pub goal_id: String,
    pub completed: bool,
    pub action_point_type: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
