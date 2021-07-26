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
use handlers::UserResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub trait User<U = String, P = String> {
    fn set_username(self, name: U);

    fn get_username(&self) -> &U;

    fn set_password(self, pass: P);

    fn get_password(&self) -> &P;
}

pub trait Session {
    type User: Serialize;
}

#[derive(Deserialize)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct AuthSession<U: Serialize, S = String> {
    user: Option<U>,
    session_id: Option<S>,
}

impl<T: Serialize> Default for AuthSession<T> {
    fn default() -> AuthSession<T> {
        AuthSession {
            user: None,
            session_id: None,
        }
    }
}

impl<U: Serialize> AuthSession<U> {
    pub fn new(session_id: String, user: U) -> Self {
        AuthSession {
            user: Some(user),
            session_id: Some(session_id),
        }
    }

    /// Retrun a reference to Some `session_id` String
    pub fn get_session_id(&self) -> &Option<String> {
        &self.session_id
    }

    /// Set `session_id` String
    pub fn set_session_id(mut self, id: String) -> Self {
        self.session_id = Some(id);
        self
    }

    /// Getter method for `user`
    pub fn get_user(&self) -> &Option<U> {
        &self.user
    }

    /// Setter method for `user`
    pub fn set_user(mut self, user: U) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_session_id(session_id: String) -> Self {
        AuthSession {
            user: None,
            session_id: Some(session_id),
        }
    }

    /// Create new `AuthSession` by passing in a user `U`
    pub fn with_user(user: U) -> Self {
        AuthSession {
            user: Some(user),
            session_id: None,
        }
    }
}

impl<T: Serialize> Session for AuthSession<T> {
    type User = T;
}
