use jsonapi::resource::Resource;

pub trait Queryset {
    type T;

    fn first(&self) -> Option<&Self::T>;
    fn all(&self) -> &Vec<Self::T>;
    fn filter(self, key: &str, value: &str) -> Self;
    fn order(self, key: &str, value: &str) -> Self;
}

pub trait Manager<T>: Send + Sync + Default + 'static {
    type Queryset: Queryset<T = T>;

    fn query(&self) -> Self::Queryset;
    fn create(&self, resource: Resource) -> T;
    fn update(&self, resource: Resource) -> T;
    fn delete(&self, resource: Resource) -> T;
}
