use super::models::{User, NewUser};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use bcrypt::{DEFAULT_COST, hash};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> Option<User> {
    use schema::user;

    // hash password
    // todo handle error better
    let hashed = match hash(password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!()
    };

    let new_user = NewUser {
        username: username,
        password: &hashed,
    };

    let user_result = super::diesel::insert(&new_user).into(user::table).get_result(conn);
    match user_result {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}

pub fn get_user_by_username(conn: &PgConnection, user_username: &str) -> Option<User> {
    use schema::user::dsl::*;

    let user_result = user.filter(username.eq(user_username))
        .load::<User>(conn);

    // If no user found returns none
    // todo get diesel to return single User rather than vec[User]
    match user_result {
        Ok(users) => match users.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None
        },
        Err(_) => None
    }
}