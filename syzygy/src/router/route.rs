use crate::endpoint::view::View;

pub enum Route<'a> {
    Item(&'a dyn View, String, Option<Vec<String>>),
    Collection(&'a dyn View, Option<Vec<String>>),
    None,
}
