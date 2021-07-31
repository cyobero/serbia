use super::auth::Auth;
use super::errors::{AuthError, FormError};
use super::handlers::{UserResponse, UserSignup};
use super::BaseUser;
use actix_web::web::Form;
use diesel::{mysql::MysqlConnection, Connection};
use failure::Fail;
use serde::Serialize;
use std::fmt::Debug;
use std::io::Error;

pub trait Valid<T: Serialize> {
    fn get_username(&self) -> Option<&str>;
    fn get_password(&self) -> Option<&str>;
    fn get_response(&self) -> &T;

    /// Returns `Ok` if `username` is not empty and `username.len()` is >= 4
    fn clean_username(&self) -> Result<&T, FormError> {
        match self.get_username() {
            None => Err(FormError::EmptyField("Username cannot be empty".to_owned())),
            Some(name) => {
                if name.len() >= 4 {
                    Ok(self.get_response())
                } else {
                    Err(FormError::FieldTooShort(
                        "Username must be at least 4 character long.".to_owned(),
                    ))
                }
            }
        }
    }

    fn clean_password(&self) -> Result<&T, FormError> {
        match self.get_password() {
            None => Err(FormError::EmptyField("Password cannot be empty".to_owned())),
            Some(name) => {
                if name.len() >= 8 {
                    Ok(self.get_response())
                } else {
                    Err(FormError::FieldTooShort(
                        "Password must be at least 8 character long.".to_owned(),
                    ))
                }
            }
        }
    }

    fn validate(&self) -> Result<&T, FormError> {
        self.clean_username().and_then(|_| self.clean_password())
    }
}
