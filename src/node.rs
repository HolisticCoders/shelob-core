use crate::attribute::{Attribute, AttributeType};
use crate::plug::Plug;

struct Node<'a, T>
where
    T: AttributeType,
{
    name: String,
    plugs: Vec<Plug<'a, T>>,       // should be a hash-map with uuids
    attributes: Vec<Attribute<T>>, // should be a hash-map with uuids
}

// related functions
impl<T> Node<'_, T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        Node {
            name: name,
            plugs: vec![],
            attributes: vec![],
        }
    }
}

// methods
impl<T> Node<'_, T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn add_attribute(&mut self, name: String, value: T) -> &Attribute<T> {
        let attribute = Attribute::new(name, value);
        self.attributes.push(attribute);
        self.attributes.last().unwrap()
    }
}
