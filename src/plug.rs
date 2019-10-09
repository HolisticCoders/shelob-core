use crate::attribute::Attribute;


pub struct Plug<'a> {
    attribute: Vec<&'a Attribute>, // TODO: needs lifetime
}