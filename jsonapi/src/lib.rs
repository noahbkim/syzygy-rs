extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod types;
pub mod resource;
pub mod document;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
