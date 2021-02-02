use jsonapi::resource::Resource;

pub struct Output {
    pub data: Resource,
    pub included: Option<Vec<Resource>>
}

impl Output {
    pub fn new(data: Resource, included: Vec<Resource>) -> Output {
        Output { data, included: Some(included) }
    }

    pub fn data(data: Resource) -> Output {
        Output { data, included: None }
    }
}

pub struct Error {

}

pub trait Serializer<T>: Send + Sync + Default + 'static {
    fn serialize(&self, object: &T) -> Result<Output, Error>;
}
