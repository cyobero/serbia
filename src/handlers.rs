use super::errors::{
    AuthError::{self, UserAlreadyExists, UserNotFound},
    FormError::{self, FieldTooShort, MismatchPasswords},
};

use super::{auth::Auth, db::*, forms::Valid, models::NewUserSession, BaseUser, DbPool};

use actix_session::Session;
use actix_web::{
    self, get,
    http::StatusCode,
    post,
    web::{self, Form},
    HttpRequest, HttpResponse,
};

use diesel::mysql::MysqlConnection;
use diesel::sql_types::Varchar;
use diesel::{sql_query, sql_types::*, RunQueryDsl};
use handlebars::Handlebars;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

impl UserSignup {
    /// Verify that `username` is at least 4 chars long.
    /// Example:
    ///     let usr = UserSignup {
    ///         username: "headbang419".to_owned(),
    ///         password: "password123".to_owned()
    ///     };
    ///     assert!(usr.clean_username().is_ok())
    pub fn clean_username(self) -> Result<BaseUser, FormError> {
        if self.username.len() < 4 {
            Err(FormError::FieldTooShort(
                "Username must be at least 4 characters long.".to_owned(),
            ))
        } else {
            Ok(BaseUser {
                username: self.username,
                password: self.password,
            })
        }
    }

    /// Verify that `password` is at least 8 chars long.
    /// Example:
    ///     let usr = UserSignup {
    ///         username: "John".to_owned(),
    ///         password: "foo".to_owned()
    ///     };
    ///     assert_eq!(usr.clean_password().is_err())
    pub fn clean_password(self) -> Result<BaseUser, FormError> {
        if self.password.len() < 8 || self.password_confirm.len() < 8 {
            Err(FormError::FieldTooShort(
                "Password must be at least 8 characters long.".to_owned(),
            ))
        } else {
            Ok(BaseUser {
                username: self.username,
                password: self.password,
            })
        }
    }

    pub fn match_passwords(self) -> Result<BaseUser, FormError> {
        if self.password == self.password_confirm {
            Ok(BaseUser {
                username: self.username,
                password: self.password,
            })
        } else {
            Err(FormError::MismatchPasswords)
        }
    }
}

impl Valid<UserSignup> for Form<UserSignup> {
    fn get_username(&self) -> Option<&str> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&str> {
        Some(&self.password)
    }

    fn get_response(&self) -> &UserSignup {
        &self.0
    }
}

impl Valid<UserLogin> for Form<UserLogin> {
    fn get_username(&self) -> Option<&str> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&str> {
        Some(&self.password)
    }

    fn get_response(&self) -> &UserLogin {
        &self.0
    }
}
/// Response for `GET /usrs/{id}`
#[derive(Debug, Serialize, Deserialize, QueryableByName, Clone)]
#[table_name = "users"]
pub struct UserResponse {
    #[sql_type = "Integer"]
    id: i32,

    #[sql_type = "Varchar"]
    username: String,

    #[sql_type = "Timestamp"]
    created_at: chrono::NaiveDateTime,
}

impl Auth for UserResponse {
    fn get_username(&self) -> Option<&String> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&String> {
        None
    }
}

/// Shared trait for cleaning forms
pub trait Clean<T: Serialize, E = FormError> {
    fn clean(self) -> Result<T, E>;
}

impl Clean<UserSignup> for Form<UserSignup> {
    /// Ensure that (1) passwords match, and (2) password length is >= 8 chars long.
    fn clean(self) -> Result<UserSignup, FormError> {
        let (p1, p2) = (self.password.to_owned(), self.password_confirm.to_owned());
        if p1 == p2 {
            if p1.len() < 8 || p2.len() < 8 {
                Err(FieldTooShort(String::from(
                    "Password must be at least 8 characters long.",
                )))
            } else {
                Ok(self.0)
            }
        } else {
            Err(MismatchPasswords)
        }
    }
}

/// Handler for resource 'GET /users/{id}
///
/// Example request:
///     `$curl localhost/users/13
///      {"id":13,"username":"testuser3","created_at":"2021-07-17T21:45:42"}`
#[get("/users/{id}")]
pub async fn retrieve_user_by_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.0.to_owned();
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    web::block(move || get_user_by_id(&conn, id))
        .await
        .map(|usr| {
            Ok(HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .json(UserResponse {
                    id: usr.id,
                    username: usr.username,
                    created_at: usr.created_at,
                }))
        })
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body(format!("{}", e))
        })
        .unwrap()
}

/// Handler for resource 'POST /users'
#[post("/signup")]
pub async fn signup(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
    form: web::Form<UserSignup>,
    sess: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    unimplemented!()

    //match input {
    //// Passwords match and meet min length requirement
    //Ok(usr) => {
    //let res = web::block(move || {
    //create_user(
    //&conn,
    //NewUser {
    //username: &usr.username,
    //password: &usr.password,
    //},
    //)
    //})
    //.await
    //.map(|_| {
    //Ok(HttpResponse::Ok()
    //.content_type("text/html; charset=utf-8")
    //.body(include_str!("../templates/signup_success.html")))
    //})
    //.map_err(|e| {
    //let data = json!({ "error": format!("{}", e) });
    //let body = hb.render("signup", &data).unwrap();
    //HttpResponse::Ok(
    //.content_type("text/html; charset=utf-8")
    //.body(&body)
    //})?;

    //res
    //}
    //// Either passwords did not match or password is too short
    //Err(e) => {
    //let data = json!({ "error": e });
    //let body = hb.render("signup", &data).unwrap();
    //Ok(HttpResponse::Ok()
    //.content_type("text/html; charset=utf-8")
    //.body(&body))
    //}
    //}
}

/// Handler for resource 'GET /users/new'
///
/// Returns form for new user signup.
#[get("/signup")]
pub async fn signup_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/signup.html")))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

/// Handler for `GET /login`
#[get("/login")]
pub async fn login_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/login.html")))
}

/// Handler for `POST /login`
#[post("/login")]
pub async fn login(
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<UserLogin>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    //let auth = form.authenticate(&conn);
    // randomly generate session id
    let session_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{UserLogin, UserSignup};
    use crate::forms::Valid;
    use actix_web::web::Form;

    #[test]
    fn user_login_is_valid() {
        let data = UserLogin {
            username: String::from("cyobero"),
            password: String::from("password123"),
        };

        let form: Form<UserLogin> = Form::<UserLogin>(data);
        assert!(form.validate().is_ok())
    }
}
