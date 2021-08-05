use super::auth::Auth;
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

impl User {
    pub fn new() -> Self {
        User {
            id: -1,
            username: String::with_capacity(255),
            password: String::with_capacity(255),
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_createted_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }
}

impl Auth for User {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }
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
