use super::auth::Auth;
use super::errors::{
    AuthError::{self, UserAlreadyExists, UserNotFound},
    FormError::{self, MismatchPasswords, PasswordTooShort},
};

use super::models::NewUser;
use super::{db::*, models::NewUserSession, AuthSession, DbPool};

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
#[derive(Serialize, Deserialize)]
pub struct NewUserInput {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

//impl Auth<NewUserInput, MysqlConnection, AuthError> for Form<NewUserInput> {
//type Output = Form<NewUserInput>;
//fn authenticate(self, conn: &MysqlConnection) -> Result<Self, AuthError> {
//let res = sql_query("SELECT * FROM users WHERE username=?")
//.bind::<Varchar, _>(&self.username)
//.execute(conn);

//// If username is not already taken
//if let Err(_) = res {
//Ok(self)
//} else {
//Err(UserAlreadyExists)
//}
//}
//}

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

/// Shared trait for cleaning forms
pub trait Clean<T, E = FormError> {
    fn clean(self) -> Result<T, E>;
}

impl Clean<NewUserInput> for Form<NewUserInput> {
    /// Ensure that (1) passwords match, and (2) password length is >= 8 chars long.
    fn clean(self) -> Result<NewUserInput, FormError> {
        let (p1, p2) = (self.password.to_owned(), self.password_confirm.to_owned());
        if p1 == p2 {
            if p1.len() < 8 || p2.len() < 8 {
                Err(PasswordTooShort)
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

    let res: Result<HttpResponse, actix_web::Error> = web::block(move || get_user_by_id(&conn, id))
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
        })?;

    res
}

/// Handler for resource 'POST /users'
#[post("/signup")]
pub async fn signup(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<DbPool>,
    form: web::Form<NewUserInput>,
    sess: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let input = form.clean();
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    match input {
        // Passwords match and meet min length requirement
        Ok(usr) => {
            let res = web::block(move || {
                create_user(
                    &conn,
                    NewUser {
                        username: &usr.username,
                        password: &usr.password,
                    },
                )
            })
            .await
            .map(|_| {
                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(include_str!("../templates/signup_success.html")))
            })
            .map_err(|e| {
                let data = json!({ "error": format!("{}", e) });
                let body = hb.render("signup", &data).unwrap();
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(&body)
            })?;

            res
        }
        // Either passwords did not match or password is too short
        Err(e) => {
            let data = json!({ "error": e });
            let body = hb.render("signup", &data).unwrap();
            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(&body))
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
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
    use super::db::create_user_session;
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

    // User found
    //  if let Ok(usr) = auth {
    //let res = web::block(move || {
    //create_user_session(
    //&conn,
    //&NewUserSession::new(&session_id.clone(), &usr.id.clone()),
    //)
    //})
    //.await
    //.map(|_| {
    //Ok(HttpResponse::Ok()
    //.content_type("text/html; charset=utf-8")
    //.body(include_str!("../templates/login_success.html")))
    //})
    //.map_err(|error| {
    //let data = json!({ "error": format!("{:?}", error) });
    //let body = hb.render("login", &data).unwrap();

    //HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
    //.content_type("text/html; charset=utf-8")
    //.body(&body)
    //})?;
    //res
    //} else {
    //let data = json!({ "error": format!("{:?}", UserNotFound) });
    //let body = hb.render("login", &data).unwrap();

    //Ok(HttpResponse::build(StatusCode::NOT_FOUND)
    //.content_type("text/html; charset=utf-8")
    //          .body(&body))
    //}
}

//impl Auth<UserLogin, AuthError> for Form<UserLogin> {
//type Output = UserResponse;
//type Connection = MysqlConnection;
//fn authenticate(self, conn: &MysqlConnection) -> Result<UserResponse, AuthError> {
//sql_query("SELECT * FROM users WHERE username=?")
//.bind::<Varchar, _>(&self.username)
//.get_result(conn)
//.map_err(|_| UserNotFound)
//}

//}
