pub mod authentication_controllers;
pub mod guards;
pub mod static_controllers;

pub use rocket_contrib::Template;
pub use rocket::request;
pub use rocket::request::{Form, FromRequest, Request};
pub use rocket::response::{Redirect, Flash};
pub use rocket::http::{Cookie, Cookies};
pub use rocket::outcome::IntoOutcome;
pub use bcrypt::{verify};
pub use std::path::{Path, PathBuf};
pub use rocket::response::NamedFile;

pub use super::forms::{authentication_forms, authentication_contexts};
pub use super::models::users;
pub use super::repositories::user_repository;
