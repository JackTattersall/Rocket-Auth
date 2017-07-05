use rocket_contrib::Template;
use rocket::request;
use rocket::request::{Form, FromRequest, Request};
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use bcrypt::{verify};
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

//--------------------------------Guards------------------------------------------------------------

#[derive(Debug)]
pub struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

//--------------------------------------------------------------------------------------------------


//-------------------------------Contexts To Render-------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct IndexContext {
    name: String,
    title: String,
    parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterContext {
    title: String,
    errors: Option<String>,
    parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    title: String,
    parent: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutContext {
    title: String,
    parent: String,
}

//--------------------------------------------------------------------------------------------------


//---------------------------------Forms------------------------------------------------------------

#[derive(FromForm)]
pub struct RegistrationForm {
    user_name: String,
    password: String,
}



#[derive(FromForm)]
pub struct LoginForm {
    user_name: String,
    password: String,
}

//--------------------------------------------------------------------------------------------------


//---------------------------------Routes-Controllers-----------------------------------------------

#[get("/")]
pub fn index(user: User) -> Template {   // Try this / route first, skip if user guard not satisfied
    let context = IndexContext{
        name: "Jack".to_string(),
        parent: "layout".to_string(),
        title: "Index".to_string()
    };
    println!("User id {}", user.0);

    Template::render("index", &context)
}

#[get("/", rank = 2)]
fn index_unauthorised() -> Redirect {
    Redirect::to("/login")
}

#[get("/register")]
pub fn register() -> Template {
    let context = RegisterContext{
        title: "Registration".to_string(),
        errors: None,
        parent: "layout".to_string()
    };

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
fn login_user(_user: User) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank=2)]
pub fn login() -> Template {
    let context = LoginContext{
        title: "Login".to_string(),
        parent: "layout".to_string()
    };

    Template::render("login", &context)
}

#[post("/login", format = "application/x-www-form-urlencoded", data = "<login>")]
pub fn login_post(mut cookies: Cookies, login: Form<LoginForm>) -> Flash<Redirect> {
    let login_form = login.get();

    // Get user
    let connection = ::repository::establish_connection();
    let user = ::repository::get_user_by_username(&connection, &login_form.user_name);

    match user {
        Some(user) => {
            match verify(&login_form.password, &user.password) {
                Ok(_) => {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    Flash::success(Redirect::to("/"), "Successfully logged in")
                },
                Err(_) => Flash::error(Redirect::to("/login"), "Incorrect Password"),
            }
        },
        None => Flash::error(Redirect::to("/login"), "Incorrect Username")
    }
}

#[get("/logged_out")]
pub fn logout(mut cookies: Cookies) -> Template {
    cookies.remove_private(Cookie::named("user_id"));

    let context = LogoutContext{
        title: "Logout".to_string(),
        parent: "layout".to_string()
    };

    Template::render("logged_out", &context)
}

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

