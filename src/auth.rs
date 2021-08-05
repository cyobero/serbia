use super::db::get_user_by_username;
use super::errors::AuthError;
use super::forms::UserSignup;
use super::users::BaseUser;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::MysqlConnection;
use serde::{Deserialize, Serialize};

pub trait Auth<T = BaseUser, C = MysqlConnection>
where
    T: Serialize + Deserialize<'static>,
{
    /// Getter method for `username`
    fn get_username(&self) -> &String;
    /// Getter method for `password`
    fn get_password(&self) -> &String;

    /// Authenticate by verifying username and password
    fn authenticate(&self, conn: &MysqlConnection) -> Result<BaseUser, AuthError> {
        self.verify_user(conn)
            .and_then(|_| self.verify_password(conn))
    }

    /// Check if user already exists in db.
    /// Example:
    ///     let usr = UserLogin {
    //       username: String::from("cyobero"),
    //       password: String::from("password123"),};
    //
    //assert!(usr.verify_user().is_ok());
    fn verify_user(&self, conn: &MysqlConnection) -> Result<BaseUser, AuthError> {
        get_user_by_username(conn, self.get_username())
            .map(|usr| BaseUser {
                id: usr.get_id().to_owned(),
                username: usr.username,
                password: usr.password,
            })
            .map_err(|_| AuthError::UserNotFound)
    }

    /// Check if password provided by user is correct.
    /// Example:
    ///     let usr = UserLogin {
    ///         username: String::from("cyobero"),
    ///         password: String::from("password123"),
    ///     };
    ///     assert!(usr.verify_password().is_ok());
    fn verify_password(&self, conn: &MysqlConnection) -> Result<BaseUser, AuthError> {
        self.verify_user(conn).and_then(|usr| {
            let hashed = hash(usr.get_password(), DEFAULT_COST).unwrap();
            let valid = verify(self.get_password(), &hashed).unwrap();
            if valid == true {
                Ok(usr)
            } else {
                Err(AuthError::InvalidPassword)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;

    #[test]
    fn user_login_authenticated() {
        use crate::forms::UserLogin;
        let conn = establish_connection().unwrap();
        let usr = UserLogin {
            username: String::from("cyobero"),
            password: String::from("password123"),
        };
        assert!(usr.authenticate(&conn).is_ok());
    }

    #[test]
    fn user_already_exists_error() {
        let usr = UserSignup {
            username: String::from("cyobero"),
            password: String::from("password123"),
            password_confirm: String::from("password123"),
        };
        let conn = establish_connection().unwrap();
        assert!(usr.verify_user(&conn).is_err());
    }
}
