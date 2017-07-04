use super::models::{User, NewUser};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use schema::user;

    let new_user = NewUser {
        username: username,
        password: password,
    };

    super::diesel::insert(&new_user).into(user::table)
        .get_result(conn)
        .expect("Error saving new post")
}