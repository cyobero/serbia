use super::errors::{
    AuthError::{self, UserAlreadyExists, UserNotFound},
    FormError::{self, FieldTooShort, MismatchPasswords},
};

use super::{auth::Auth, db::*, forms::Valid, BaseUser, DbPool};

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
    /// Verify that confirmation password matches input password.
    /// Example:
    ///     let usr = UserSignup {
    ///         username: "cyobero"
    ///     }
    pub fn match_passwords(self) -> Result<BaseUser, FormError> {
        if self.password == self.password_confirm {
            Ok(BaseUser {
                id: None,
                username: self.username,
                password: self.password,
            })
        } else {
            Err(FormError::MismatchPasswords)
        }
    }
}

impl Valid for Form<UserSignup> {
    fn get_username(&self) -> Option<&str> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&str> {
        Some(&self.password)
    }

    fn get_response(&self) -> BaseUser {
        BaseUser {
            id: None,
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        }
    }
}

impl Valid for Form<UserLogin> {
    fn get_username(&self) -> Option<&str> {
        Some(&self.username)
    }

    fn get_password(&self) -> Option<&str> {
        Some(&self.password)
    }

    fn get_response(&self) -> BaseUser {
        BaseUser {
            id: None,
            username: self.username.to_owned(),
            password: self.password.to_owned(),
        }
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

    // Validate form (i.e. check for empty fields, min. field length requirements, etc.)
    let valid = form.validate();

    match valid {
        // Form is valid
        Ok(usr) => match usr.authenticate(&conn) {
            Ok(u) => {
                let _session_id: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect();

                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(include_str!("../templates/login_success.html")))
            }
            Err(e) => {
                let data = json!({ "error": e });
                let body = hb.render("login", &data).unwrap();
                Ok(HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body(&body))
            }
        },

        // Form is not valid
        Err(fe) => {
            let data = json!({ "error": fe });
            let body = hb.render("login", &data).unwrap();
            Ok(HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(&body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UserLogin, UserSignup};
    use crate::auth::Auth;
    use crate::forms::Valid;
    use actix_web::web::Form;

    #[test]
    fn user_not_exist_error() {
        use crate::db::establish_connection;
        let data = UserLogin {
            username: String::from("iamnotreal"),
            password: String::from("password123"),
        };

        let conn = establish_connection().unwrap();
        let form: Form<UserLogin> = Form::<UserLogin>(data);
        assert!(form.authenticate(&conn).is_err());
    }

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
