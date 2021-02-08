pub struct Request {
    pub request: hyper::Request<hyper::Body>,
    pub parents: Option<Vec<String>>,
}

impl Request {

}
