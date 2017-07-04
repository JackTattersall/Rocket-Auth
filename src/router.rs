extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;


use self::rocket_contrib::Template;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use bcrypt::{verify};

#[derive(Serialize, Deserialize)]
pub struct IndexContext {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterContext {
    title: String,
    errors: Option<String>,
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

#[derive(FromForm)]
pub struct LoginForm {
    user_name: String,
    password: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = IndexContext{ name: "Jack".to_string() };

    Template::render("index", &context)
}
// implement this when i have the 0.3.0
//
//#[get("/", rank = 2)]
//fn index_unauthorised() -> Redirect {
//    Redirect::to("/login")
//}

#[get("/register")]
pub fn register() -> Template {
    let context = RegisterContext{ title: "Registration Page".to_string(), errors: None };

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

#[post("/login", format = "application/x-www-form-urlencoded", data = "<login>")]
pub fn login_post(mut cookies: &Cookies, login: Form<LoginForm>) -> Flash<Redirect> {
    let login_form = login.get();

    // Get user
    let connection = ::repository::establish_connection();
    let user = ::repository::get_user_by_username(&connection, &login_form.user_name);

    match user {
        Some(user) => {
            match verify(&login_form.password, &user.password) {
                Ok(valid) => {
                    println!("{}", valid);
                    cookies.add(Cookie::new("session_key", "123456787654321"));
                    Flash::success(Redirect::to("/"), "Successfully logged in")
                },
                Err(_) => Flash::error(Redirect::to("/login"), "Incorrect Password"),
            }
        },
        None => Flash::error(Redirect::to("/login"), "Incorrect Username")
    }
}

#[get("/logged_out")]
pub fn logout(cookies: &Cookies) -> Template {
    cookies.remove("session_key");
    Template::render("logged_out", &"")
}

// todo Get the 0.3.0 Rocket straight from github
// change &Cookies to Cookies
// implement private session cookies and a
