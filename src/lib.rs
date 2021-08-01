#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate failure;

pub mod auth;
pub mod db;
pub mod errors;
pub mod forms;
pub mod handlers;
pub mod models;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub trait Session {
    type User: Serialize;
}

#[derive(Deserialize)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct BaseUser {
    id: Option<i32>,
    username: String,
    password: String,
}

impl BaseUser {
    pub fn new() -> Self {
        BaseUser::default()
    }
}

impl Default for BaseUser {
    fn default() -> BaseUser {
        BaseUser {
            id: None,
            username: String::new(),
            password: String::new(),
        }
    }
}
