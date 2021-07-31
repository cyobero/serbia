//! Module for database interactions
//
use super::models::{NewUser, NewUserSession, User};
use super::schema::{sessions, users};
use diesel::{mysql::MysqlConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Serialize;
use std::env;

type DieselError = diesel::result::Error;

/// Returns all user records from db.
///
/// Examples:
///     let users = get_all_users(pool).unwrap();
///     assert_ne!(users.len(), 0);  // Passes if users already exist in db
pub fn get_all_users(conn: &MysqlConnection) -> Result<Vec<User>, DieselError> {
    let items = users::table.get_results(conn)?;
    Ok(items)
}

/// Create new user record in db.
///
/// Example:
///     let username = String::from("testuser2");
///     let password = String::from("password123");
///     let usr = create_user(&conn, NewUser { username, password });
///     assert!(usr.is_ok()); // only passes the if `usr` not already in db
pub fn create_user(conn: &MysqlConnection, item: NewUser) -> Result<usize, DieselError> {
    diesel::insert_into(users::table)
        .values(&item)
        .execute(conn)
}

/// Query db for user with given `id`.
/// Return a `User` struct if found.
///
/// Example:
///     let id = 13;
///     let res = get_user_by_id(&conn, id).unwrap(); // Returns a `User` struct.
///     assert_eq!(res.id, id);
pub fn get_user_by_id(conn: &MysqlConnection, id_: i32) -> Result<User, DieselError> {
    let res = users::table.filter(users::id.eq(id_)).get_result(conn)?;
    Ok(res)
}

/// Query db for user with given `username` and return `User` struct if found.
///
/// Example:
///     let query_user = String::from("stewiedewie69");
///     let res = get_user_by_username(&conn, query_user).unwrap();
///     assert_eq!(res.username, "stewiedewie69");
pub fn get_user_by_username(conn: &MysqlConnection, username_: &str) -> Result<User, DieselError> {
    let res = users::table
        .filter(users::username.eq(username_))
        .get_result(conn)?;
    Ok(res)
}

/// Removes user with given `id` from db.
///
/// Example:
///     let id = 15;
///     let res = remove_user_by_id(&conn, id);
///     assert!(res.is_ok())
pub fn remove_user_by_id(conn: &MysqlConnection, id_: i32) -> Result<usize, DieselError> {
    let res = diesel::delete(users::table)
        .filter(users::id.eq(id_))
        .execute(conn)?;
    Ok(res)
}

/// Establishes connection to db and returns `MysqlConnection` instance
pub fn establish_connection() -> Result<MysqlConnection, diesel::ConnectionError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&db_url)
}

/// Removes user with given `username` from db
pub fn remove_user_by_username(
    conn: &MysqlConnection,
    username_: String,
) -> Result<usize, DieselError> {
    let res = diesel::delete(users::table)
        .filter(users::username.eq(username_))
        .execute(conn)?;
    Ok(res)
}

/// Create new `session` record in database
/// Example:
///     let token = String::from("test-token");
///     let new_session = create_user_session(&conn, user_id_:42, session_token: token);
///     assert_eq!(new_session, Ok(1));
pub fn create_user_session(
    conn: &MysqlConnection,
    session: &NewUserSession,
) -> Result<usize, DieselError> {
    let res = diesel::insert_into(sessions::table)
        .values(session)
        .execute(conn)?;
    Ok(res)
}

/// End current user session
pub fn end_user_session(
    conn: &MysqlConnection,
    session_key: &str,
) -> Result<usize, std::io::Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::establish_connection;
    use crate::models::NewUser;

    #[test]
    fn session_created() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key: u32 = rng.gen();
        let conn = establish_connection().unwrap();
    }

    #[test]
    fn user_removed_by_username() {
        use super::remove_user_by_username;
        let conn = establish_connection().unwrap();
        let name = String::from("stewiedewie69");
        let res = remove_user_by_username(&conn, name);
        assert!(res.is_ok());
    }

    #[test]
    fn user_removed_by_id() {
        use super::remove_user_by_id;
        let conn = establish_connection().unwrap();
        let id = 15;
        let res = remove_user_by_id(&conn, id).unwrap();
    }

    #[test]
    fn retrieved_by_id() {
        use super::get_user_by_id;
        let conn = establish_connection().unwrap();
        let q = 13;
        let res = get_user_by_id(&conn, q).unwrap();
        assert_eq!(res.id, q);
    }

    #[test]
    fn retrieved_by_username() {
        use super::get_user_by_username;
        let conn = establish_connection().unwrap();
        let query_user = String::from("bender3000");
        let res = get_user_by_username(&conn, &query_user).unwrap();
        assert_eq!(res.username, "bender3000");
    }

    #[test]
    fn user_created_and_removed() {
        use super::{create_user, remove_user_by_username};
        let conn = establish_connection().unwrap();
        let _ = remove_user_by_username(&conn, "testuser1".to_owned());
        let item = NewUser {
            username: "testuser1",
            password: "testpassword123",
        };

        let usr = create_user(&conn, item);
        assert!(usr.is_ok());
    }
}
