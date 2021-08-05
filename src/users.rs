use super::auth::Auth;
use super::models::User;

use chrono::prelude::*;
use diesel::sql_types::{Integer, Timestamp, Varchar};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BaseUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl BaseUser {
    pub fn new() -> Self {
        BaseUser::default()
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    fn _get_username(&self) -> &String {
        &self.username
    }

    fn _get_password(&self) -> &String {
        &self.password
    }
}

impl Auth for BaseUser {
    fn get_username(&self) -> &String {
        self._get_username()
    }

    fn get_password(&self) -> &String {
        self._get_password()
    }
}

impl Default for BaseUser {
    fn default() -> BaseUser {
        BaseUser {
            id: -1,
            username: String::with_capacity(255),
            password: String::with_capacity(255),
        }
    }
}

/// Response for `GET /usrs/{id}`
#[derive(Debug, Serialize, Deserialize, QueryableByName, Clone)]
#[table_name = "users"]
pub struct UserResponse {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "Varchar"]
    pub username: String,

    #[sql_type = "Timestamp"]
    pub created_at: chrono::NaiveDateTime,
}

impl UserResponse {
    pub fn new() -> Self {
        UserResponse::default()
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_createted_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }
}

impl Default for UserResponse {
    fn default() -> UserResponse {
        UserResponse {
            id: -1,
            username: String::with_capacity(255),
            created_at: Utc::now().naive_utc(),
        }
    }
}
