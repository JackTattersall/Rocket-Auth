#![feature(plugin, custom_derive)]
#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(const_fn)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub mod router;
pub mod models;
pub mod schema;
pub mod repository;

extern crate rocket;
extern crate serde_json;
extern crate serde;
extern crate dotenv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
