#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

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
use serde::Deserialize;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Deserialize)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}
