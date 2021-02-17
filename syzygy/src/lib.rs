extern crate async_trait;
extern crate hyper;
extern crate jsonapi;

pub mod router;
pub mod types;
pub mod view;
pub mod helper;
pub mod middleware;

pub type Request = hyper::Request<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
