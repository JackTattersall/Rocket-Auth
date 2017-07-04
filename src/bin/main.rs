#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_auth;

fn main() {
    rocket::ignite().mount("/", routes![
    rocket_auth::router::index, rocket_auth::router::register, rocket_auth::router::login,
    rocket_auth::router::register_post, rocket_auth::router::login_post
    ]).launch();
}