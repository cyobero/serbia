use super::auth::Auth;
use super::forms::{UserLogin, UserSignup, Valid};
use super::users::{BaseUser, UserResponse};
use super::{db::*, DbPool};

use actix_session::Session;
use actix_web::{
    self, get,
    http::StatusCode,
    post,
    web::{self, Form, HttpRequest},
    HttpResponse,
};

use diesel::{sql_query, sql_types::*, RunQueryDsl};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

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
                    id: usr.get_id().to_owned(),
                    username: usr.get_username().to_owned(),
                    created_at: usr.get_createted_at().to_owned(),
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
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    let valid = form.validate();

    match valid {
        Ok(usr) => web::block(move || usr.authenticate(&conn))
            .await
            .map(|_| {
                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(include_str!("../templates/signup_success.html")))
            })
            .map_err(|e| {
                let data = json!({ "error": format!("{}", e) });
                let body = hb.render("signup", &data).unwrap();
                HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body(&body)
            })?,

        Err(e) => {
            let data = json!({ " error": e });
            let body = hb.render("signup", &data).unwrap();
            Ok(HttpResponse::InternalServerError()
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
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool
        .get()
        .expect("Could not establish connection from pool.");

    let valid = form.validate();

    match valid {
        Ok(usr) => web::block(move || usr.authenticate(&conn))
            .await
            .map(|u| {
                session.set("user", &u)?;

                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(include_str!("../templates/login_success.html")))
            })
            .map_err(|e| {
                let data = json!({ "error": format!("{}", e) });
                let body = hb.render("login", &data).unwrap();
                HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body(&body)
            })?,
        Err(e) => {
            let data = json!({ "error": e });
            let body = hb.render("login", &data).unwrap();
            Ok(HttpResponse::InternalServerError()
                .content_type("text/html; charset=utf-8")
                .body(&body))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::Auth;
    use crate::forms::{UserLogin, Valid};
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
