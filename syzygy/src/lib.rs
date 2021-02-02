extern crate jsonapi;
extern crate tide;
extern crate route_recognizer;
extern crate async_trait;

pub mod endpoint;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
