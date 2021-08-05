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
pub mod users;

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
