#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_auth;
extern crate rocket_contrib;

use rocket_contrib::Template;

fn main() {
    rocket::ignite().mount("/", routes![
    rocket_auth::router::index, rocket_auth::router::register, rocket_auth::router::login,
    rocket_auth::router::register_post, rocket_auth::router::login_post, rocket_auth::router::logout,
    rocket_auth::router::index_unauthorised, rocket_auth::router::login_user,
    rocket_auth::router::all
    ]).attach(Template::fairing()).launch();
}