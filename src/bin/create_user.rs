use blog_user::db::{create_user, establish_connection};
use blog_user::models::NewUser;
use clap::{App, Arg};

fn main() {
    let conn = establish_connection().expect("Failed to establish connection.");

    let matches = App::new("Create New User")
        .author("Czar Yobero")
        .arg(
            Arg::with_name("username")
                .short("u")
                .index(1)
                .long("username")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .index(2)
                .takes_value(true),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    let _ = create_user(&conn, NewUser { username, password }).expect("Failed to create user.");

    println!("User successfully created!");
}
