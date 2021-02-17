pub trait Method {
    fn allowed(&self) -> bool {
        return false;
    }
}
