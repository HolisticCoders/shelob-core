use crate::attribute::Attribute;
use crate::plug::Plug;

struct Node<'a> {
    name: String,
    plugs: Vec<Plug<'a>>,
    attributes: Vec<Box<Attribute>>,
}