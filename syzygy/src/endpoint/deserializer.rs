use jsonapi::resource::Resource;

pub struct Input<'a> {
    pub data: Resource,
    pub included: Option<&'a [Resource]>
}

pub struct Error {

}

pub trait Deserializer: Send + Sync + Default + 'static {
    type T;

    fn deserialize(&self, input: Input) -> Result<Self::T, Error>;
}
