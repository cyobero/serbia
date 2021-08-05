use super::auth::Auth;
use super::db::get_user_by_username;
use super::errors::AuthError;
use super::errors::FormError;
use super::users::BaseUser;

use actix_web::web::Form;
use diesel::{mysql::MysqlConnection, Connection};
use serde::{Deserialize, Serialize};

pub trait Valid<T = BaseUser>
where
    T: Serialize,
{
    fn get_username(&self) -> &String;
    fn get_password(&self) -> &String;
    fn get_response(&self) -> T;

    /// Returns `Ok` if `username` is not empty and `username.len()` is >= 4
    fn clean_username(&self) -> Result<T, FormError> {
        if self.get_username().len() >= 4 {
            Ok(self.get_response())
        } else {
            let e = FormError::FieldTooShort(String::from(
                "Username must be at least 4 characters long.",
            ));
            Err(e)
        }
    }

    /// Retrusns `Ok` if `password` is not empty and `username.len()` is >= 8
    fn clean_password(&self) -> Result<T, FormError> {
        if self.get_password().len() >= 8 {
            Ok(self.get_response())
        } else {
            let e = FormError::FieldTooShort(String::from(
                "Field 'password' must be at least 8 characters long.",
            ));
            Err(e)
        }
    }

    fn validate(&self) -> Result<T, FormError> {
        self.clean_username().and_then(|_| self.clean_password())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl Auth for UserLogin {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }
}

impl Valid<UserLogin> for UserLogin {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_response(&self) -> Self {
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

impl UserSignup {
    /// Verify that confirmation password matches input password.  Example:
    ///     let usr = UserSignup {
    ///         username: "cyobero"
    ///     }
    pub fn match_passwords(self) -> Result<BaseUser, FormError> {
        if self.password == self.password_confirm {
            Ok(BaseUser {
                id: -1,
                username: self.get_username().to_owned(),
                password: self.get_password().to_owned(),
            })
        } else {
            Err(FormError::MismatchPasswords)
        }
    }
}

impl Auth for UserSignup {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn verify_user(&self, conn: &MysqlConnection) -> Result<BaseUser, AuthError> {
        let usr = get_user_by_username(conn, self.get_username());

        match usr {
            Ok(_) => Err(AuthError::UserAlreadyExists),
            Err(_) => Ok(BaseUser {
                id: usr.unwrap().get_id().to_owned(),
                username: self.get_username().to_owned(),
                password: self.get_password().to_owned(),
            }),
        }
    }
}

impl Valid<UserSignup> for Form<UserSignup> {
    fn get_username(&self) -> &String {
        &self.username
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_response(&self) -> UserSignup {
        self.0.to_owned()
    }
}
