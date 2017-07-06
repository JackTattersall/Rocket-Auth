pub mod authentication_forms;
pub mod authentication_contexts;

pub use rocket_contrib::Template;
pub use rocket::request;
pub use rocket::request::{Form, FromRequest, Request};
pub use rocket::response::{Redirect, Flash};
pub use rocket::http::{Cookie, Cookies};
pub use rocket::outcome::IntoOutcome;
pub use bcrypt::{verify};
pub use std::path::{Path, PathBuf};
pub use rocket::response::NamedFile;
