use super::diesel::Connection;
use super::errors::FormError;

use chrono::{DateTime, Utc};
use serde::Serialize;

pub trait Auth<T, C, E = FormError>
where
    T: Serialize,
    C: Connection,
{
    type Output;
    fn authenticate(self, conn: &C) -> Result<Self::Output, E>;
}

pub trait Validate<I, C, E = FormError>
where
    I: Serialize,
    C: Connection,
{
    type Output;
    fn validate(self) -> Result<Self::Output, E>;
}

pub trait Session<T> {
    type Sess;
    fn set_session_token(self, token: T) -> Self::Sess;
    fn get_session_token(&self) -> &T;
}

pub struct NewUserSession {
    pub user_id: i32,
    pub begin: DateTime<Utc>,
    pub actix_session: Option<String>,
}

pub struct ActixSession {
    pub user_id: i32,
    pub begin: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub actix_session: String,
}

impl ActixSession {
    pub fn new() -> Self {
        ActixSession::default()
    }

    /// Create new `ActixSession` by passing in a `token` `String`.
    pub fn from_string(token: String) -> Self {
        ActixSession {
            user_id: -1,
            begin: Utc::now(),
            end: None,
            actix_session: token,
        }
    }

    /// Setter method for `user_id`
    pub fn set_user_id(mut self, id: i32) -> Self {
        self.user_id = id;
        self
    }

    /// Getter method for `user_id`
    pub fn get_user_id(&self) -> Option<i32> {
        Some(self.user_id)
    }
}

impl Session<String> for ActixSession {
    type Sess = ActixSession;
    fn set_session_token(mut self, token: String) -> Self {
        self.actix_session = token;
        self
    }

    fn get_session_token(&self) -> &String {
        &self.actix_session
    }
}

impl Default for ActixSession {
    fn default() -> ActixSession {
        ActixSession {
            user_id: -1,
            begin: Utc::now(),
            end: None,
            actix_session: String::new(),
        }
    }
}
