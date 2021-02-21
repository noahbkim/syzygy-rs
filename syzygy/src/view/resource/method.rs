use crate::view::actions::Disallowed;

pub trait Method {
    fn allowed(&self) -> bool;
}

impl Method for Disallowed {
    fn allowed(&self) -> bool {
        return false;
    }
}
