use super::db::get_user_by_username;
use super::errors::AuthError;
use super::handlers::UserLogin;
use super::handlers::UserResponse;

use actix_web::web::Form;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::{Connection, MysqlConnection};
use serde::Serialize;

pub trait Auth<C = MysqlConnection>
where
    C: Connection,
{
    type User: Serialize;
    type Error;

    fn authenticate(&self, conn: &C) -> Result<Self::User, Self::Error>;

    /// Check to see if input `password` matches hashed passowrd
    fn verify_password(self, conn: &MysqlConnection) -> Result<Self::User, AuthError>;

    /// Check to see if `username` already exists
    fn verify_username(&self, conn: &MysqlConnection) -> Result<Self::User, AuthError>;
}

impl Auth for UserLogin {
    type User = UserLogin;
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

#[cfg(test)]
mod tests {
    use crate::db::establish_connection;

    #[test]
    fn user_authenticated() {
        use super::Auth;
        use super::UserLogin;
        let usr = UserLogin {
            username: "cyobero".to_owned(),
            password: "password123".to_owned(),
        };
        let conn = establish_connection().unwrap();
        let res = usr.authenticate(&conn);
        assert!(res.is_ok());
    }
}
