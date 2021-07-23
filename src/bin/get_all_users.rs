use blog_user::db::{establish_connection, get_all_users};

fn main() {
    let conn = establish_connection().expect("Failed to establish connection.");
    let res = get_all_users(&conn).expect("Failed to retrieve users.");
    res.into_iter().for_each(|usr| println!("{:?}", usr));
}
