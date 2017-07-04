#![feature(plugin)]
#![feature(attr_literals)]
#![feature(custom_attribute)]
#![feature(const_fn)]
#![plugin(rocket_codegen)]
pub mod router;
extern crate rocket;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
