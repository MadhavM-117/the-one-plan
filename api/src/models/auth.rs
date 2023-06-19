use crate::schema::*;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl From<User> for UserData {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            phone_number: u.phone_number,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = auth_sessions)]
pub struct AuthSession {
    pub id: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
