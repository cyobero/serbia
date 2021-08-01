use super::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::{sql_types::*, Expression, Insertable};
use serde::Serialize;

#[derive(Debug, Serialize, Queryable, QueryableByName)]
pub struct User {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Varchar"]
    pub username: String,
    #[sql_type = "Varchar"]
    pub password: String,
    #[sql_type = "Timestamp"]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'nu> {
    pub username: &'nu str,
    pub password: &'nu str,
}

#[derive(Debug, Insertable, Serialize)]
#[table_name = "sessions"]
pub struct NewUserSession {
    pub session_key: String,
    pub user_id: i32,
}

impl NewUserSession {
    pub fn new(session_key: String, user_id: i32) -> Self {
        NewUserSession {
            session_key,
            user_id,
        }
    }
}

#[derive(Debug, Serialize, Queryable, QueryableByName)]
pub struct UserSession {
    #[sql_type = "Varchar"]
    session_key: String,

    #[sql_type = "Integer"]
    user_id: i32,
}
