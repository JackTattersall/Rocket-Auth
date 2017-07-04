extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;

use self::rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct IndexContext {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterContext {
    title: String,
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

#[get("/login")]
pub fn login() -> Template {
    let context = LoginContext{ title: "Login Page".to_string() };

    Template::render("login", &context)
}
