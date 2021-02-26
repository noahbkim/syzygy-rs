pub enum Action {
    Create,
    List,
    Retrieve(String),
    Update(String),
    Delete(String),
}
