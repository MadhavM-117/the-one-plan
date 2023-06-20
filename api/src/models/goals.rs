use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use chrono::{NaiveDateTime, Utc};
use crate::schema::{goals, action_points};
use crate::utils::new_id;

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


impl Default for Goal {
    fn default() -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: new_id(),
            user_id: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            created_at: now.clone(),
            updated_at: now.clone()
        }
    }
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

impl Default for ActionPoint {
    fn default() -> Self {
        let now = Utc::now().naive_utc();

        Self {
            id: new_id(),
            goal_id: "".to_string(),
            completed: false,
            action_point_type: "".to_string(),
            description: "".to_string(),
            created_at: now.clone(),
            updated_at: now.clone()
        }
    }
}
