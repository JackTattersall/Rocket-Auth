extern crate rocket;

#[get("/")]
pub const fn index() -> &'static str {
    "Hello, world!"
}

#[get("/register")]
pub const fn register() -> &'static str {
    "Registration page"
}

#[get("/login")]
pub const fn login() -> &'static str {
    "Login page"
}