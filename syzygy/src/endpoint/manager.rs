use jsonapi::resource::Resource;

pub trait Queryset {
    type T;

    fn first(&self) -> Option<&Self::T>;
    fn all(&self) -> &Vec<Self::T>;
    fn filter(self, key: &str, value: &str) -> Self;
    fn order(self, key: &str, value: &str) -> Self;
}

pub trait Manager: Send + Sync + Default + 'static {
    type T;
    type Queryset: Queryset<T = Self::T>;

    fn query(&self) -> Self::Queryset;
    fn create(&self, resource: Resource) -> Self::T;
    fn update(&self, resource: Resource) -> Self::T;
    fn delete(&self, resource: Resource) -> Self::T;
}
