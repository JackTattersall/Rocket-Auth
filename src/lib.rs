#![feature(plugin, custom_derive)]
#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(const_fn)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod controllers;
pub mod models;
pub mod repositories;
pub mod forms;

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
extern crate serde;
extern crate dotenv;
extern crate bcrypt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
