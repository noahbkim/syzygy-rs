extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod types;
pub mod resource;


pub trait JsonApiResource: serde::Serialize where for<'de> Self: serde::Deserialize<'de> {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
