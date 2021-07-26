use super::db::get_user_by_username;
use super::errors::AuthError;
use super::handlers::UserLogin;
use super::handlers::UserResponse;
use crate::User;

use actix_web::web::Form;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::{Connection, MysqlConnection};
use serde::Serialize;

pub trait Auth<C = MysqlConnection>
where
    C: Connection,
{
    type Input: User;
    type Error;

    fn authenticate(&self, conn: &C) -> Result<Self::Input, Self::Error>;

    /// Check to see if input `password` matches hashed passowrd
    fn verify_password(self, conn: &MysqlConnection) -> Result<Self::Input, AuthError>;

    /// Check to see if `username` already exists
    fn verify_username(&self, conn: &MysqlConnection) -> Result<Self::Input, AuthError>;
}

impl Auth for UserLogin {
    type Input = UserLogin;
    type Error = AuthError;

    fn authenticate(&self, conn: &MysqlConnection) -> Result<Self, AuthError> {
        self.verify_username(conn)
            .and_then(|u| u.verify_password(conn))
    }

    fn verify_password(self, conn: &MysqlConnection) -> Result<Self, AuthError> {
        let usr = self.verify_username(conn)?;
        let hashed = hash(&self.password, DEFAULT_COST).expect("Could not hash password.");
        let valid = verify(usr.password, &hashed).unwrap();
        if valid == true {
            Ok(self)
        } else {
            Err(AuthError::InvalidPassword)
        }
    }

    fn verify_username(&self, conn: &MysqlConnection) -> Result<Self, AuthError> {
        get_user_by_username(conn, &self.username)
            .map(|u| UserLogin {
                username: u.username,
                password: u.password,
            })
            .map_err(|_| AuthError::UserNotFound)
    }
}

impl User for UserLogin {
    fn set_username(mut self, name: String) {
        self.username = name;
    }

    fn get_username(&self) -> &String {
        &self.username
    }

    fn set_password(mut self, pass: String) {
        self.password = pass;
    }

    fn get_password(&self) -> &String {
        &self.password
    }
}

#[cfg(test)]
mod tests {
    use crate::db::establish_connection;

    #[test]
    fn user_authenticated() {
        use super::UserLogin;
        use super::{Auth, User};
        let usr = UserLogin {
            username: "cyobero".to_owned(),
            password: "password123".to_owned(),
        };
        let conn = establish_connection().unwrap();
        assert!(usr.authenticate(&conn).is_ok());
        usr.set_username("nonexistent".to_owned());
    }
}
