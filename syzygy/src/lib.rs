extern crate jsonapi;
extern crate hyper;
extern crate async_trait;

pub mod endpoint;
pub mod types;
pub mod router;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
