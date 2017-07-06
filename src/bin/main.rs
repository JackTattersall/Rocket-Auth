#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_auth;
extern crate rocket_contrib;

use rocket_auth::controllers::{static_controllers, authentication_controllers};

use rocket_contrib::Template;

fn main() {
    rocket::ignite().mount("/", routes![
    authentication_controllers::login,
    authentication_controllers::register,
    authentication_controllers::index,
    authentication_controllers::index_unauthorised,
    authentication_controllers::login_post,
    authentication_controllers::login_user,
    authentication_controllers::logout,
    authentication_controllers::register_post,
    static_controllers::all
    ]).attach(Template::fairing()).launch();
}
