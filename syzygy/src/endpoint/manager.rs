use jsonapi::resource::Resource;

pub trait Queryset<T> {
    fn first(&self) -> Option<&T>;
    fn all(&self) -> &Vec<T>;
    fn filter(self, key: &str, value: &str) -> Self;
    fn order(self, key: &str, value: &str) -> Self;
}

pub trait Manager<T>: Send + Sync + Default + 'static {
    type Queryset: Queryset<T>;
    fn query(&self) -> Self::Queryset;
    fn create(&self, resource: Resource) -> T;
    fn update(&self, resource: Resource) -> T;
    fn delete(&self, resource: Resource) -> T;
}
