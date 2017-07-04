#![feature(plugin)]
#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(const_fn)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;

pub mod router;

extern crate rocket;
extern crate serde_json;
extern crate serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
