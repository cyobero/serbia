#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use actix_session::{CookieSession, Session};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use blog_user::handlers;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use handlebars::Handlebars;

use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    db: web::Data<DbPool>,
}

/// Handler for index page
#[get("/")]
pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    request: HttpRequest,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    println!("{:?}", &request);
    let cookie = request.headers().get("cookie").unwrap();
    let data = json!({
        "session_id": session.get::<u32>("session-id").unwrap(),
        "cookie": format!("{:?}", &cookie),
    });
    let body = hb
        .render_template(include_str!("../templates/index.html"), &data)
        .unwrap();
    Ok(HttpResponse::Ok().body(&body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // Create database pool
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // For template rendering
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "../templates/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // Start HTTP server
    let address = "127.0.0.1:8000"; // address and port number to serve app
    println!("Serving at {}", &address);
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(index)
            .service(handlers::signup)
            .service(handlers::signup_form)
            .service(handlers::retrieve_user_by_id)
            .service(handlers::login)
            .service(handlers::login_form)
    })
    .bind(&address)?
    .run()
    .await
}
