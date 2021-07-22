use super::auth::Session;
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

#[derive(Debug, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
}

impl NewSession {
    pub fn new(user_id: i32, token: String) -> Self {
        NewSession { user_id, token }
    }
}

impl Default for NewSession {
    fn default() -> NewSession {
        NewSession {
            user_id: -1,
            token: "Default Session".to_owned(),
        }
    }
}

impl Session<String> for NewSession {
    type Sess = NewSession;
    fn set_session_token(mut self, token: String) -> Self {
        self.token = token;
        self
    }

    fn get_session_token(&self) -> &String {
        &self.token
    }
}
