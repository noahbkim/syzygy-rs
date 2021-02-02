use jsonapi::resource::Resource;

pub struct Input<'a> {
    pub data: Resource,
    pub included: Option<&'a [Resource]>
}

pub struct Error {

}

pub trait Deserializer<T>: Send + Sync + Default + 'static {
    fn deserialize(&self, input: Input) -> Result<T, Error>;
}
