use super::db::{establish_connection, get_user_by_username};
use super::errors::AuthError;
use super::handlers::{UserLogin, UserResponse, UserSignup};
use super::BaseUser;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::MysqlConnection;

pub trait Auth<C = MysqlConnection> {
    /// Getter method for `username`
    fn get_username(&self) -> Option<&String>;
    /// Getter method for `password`
    fn get_password(&self) -> Option<&String>;

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
        get_user_by_username(conn, self.get_username().unwrap())
            .map(|usr| BaseUser {
                id: Some(usr.id),
                username: usr.username,
                password: usr.password,
            })
            .map_err(|_| AuthError::UserNotFound)
    }

    fn verify_password(&self, conn: &MysqlConnection) -> Result<BaseUser, AuthError> {
        self.verify_user(conn).and_then(|usr| {
            let hashed = hash(&usr.password, DEFAULT_COST).unwrap();
            let valid = verify(self.get_password().unwrap(), &hashed).unwrap();
            if valid == true {
                Ok(usr)
            } else {
                Err(AuthError::InvalidPassword)
            }
        })
    }
}

impl Auth for UserLogin {
    fn get_username(&self) -> Option<&String> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&String> {
        Some(&self.password)
    }
}

impl Auth for UserSignup {
    fn get_username(&self) -> Option<&String> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&String> {
        Some(&self.password)
    }
}

impl Auth for BaseUser {
    fn get_username(&self) -> Option<&String> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&String> {
        Some(&self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_login_authenticated() {
        let conn = establish_connection().unwrap();
        let usr = UserLogin {
            username: String::from("cyobero"),
            password: String::from("password123"),
        };
        assert!(usr.authenticate(&conn).is_ok());
    }
}
