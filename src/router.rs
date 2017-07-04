extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;


use self::rocket_contrib::Template;
use rocket::request::{Form, FromForm};
use rocket::response::Redirect;

#[derive(Serialize, Deserialize)]
pub struct IndexContext {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterContext {
    title: String,
}

#[derive(FromForm)]
pub struct RegistrationForm {
    user_name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    title: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = IndexContext{ name: "Jack".to_string() };

    Template::render("index", &context)
}

#[get("/register")]
pub fn register() -> Template {
    let context = RegisterContext{ title: "Registration Page".to_string() };

    Template::render("register", &context)
}

#[post("/register", format = "application/x-www-form-urlencoded", data = "<registration>")]
pub fn register_post(registration: Form<RegistrationForm>) -> Redirect {
    let registration_form = registration.get();

    // todo Handle errors, verification etc
    let connection = ::repository::establish_connection();

    let result = ::repository::create_user(
        &connection, &registration_form.user_name, &registration_form.password
    );

    match result {
        Some(_) => Redirect::to("/login"),
        None => Redirect::to("/register")
    }
}

#[get("/login")]
pub fn login() -> Template {
    let context = LoginContext{ title: "Login Page".to_string() };

    Template::render("login", &context)
}
