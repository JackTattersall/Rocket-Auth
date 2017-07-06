use super::*;

#[derive(FromForm)]
pub struct RegistrationForm {
    pub user_name: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct LoginForm {
    pub user_name: String,
    pub password: String,
}