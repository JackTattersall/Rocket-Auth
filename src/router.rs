extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;

use self::rocket_contrib::Template;

#[derive(Serialize, Deserialize)]
pub struct TemplateContext {
    name: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = TemplateContext{ name: "Jack".to_string() };

    Template::render("index", &context)
}

#[get("/register")]
pub fn register() -> &'static str {
    "Registration page"
}

#[get("/login")]
pub fn login() -> &'static str {
    "Login page"
}
