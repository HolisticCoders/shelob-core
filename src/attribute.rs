use crate::dg::{DGITemType, DGItem, DGPath};

pub trait AttributeType {}

pub struct Attribute<T>
where
    T: AttributeType,
{
    pub name: String,
    pub node: DGPath,
    pub value: T,
}

impl<T> Attribute<T>
where
    T: AttributeType,
{
    pub fn new(name: String, node: DGPath, value: T) -> Self {
        Attribute {
            name: name,
            node: node,
            value: value,
        }
    }
}

impl<T> DGItem for Attribute<T>
where
    T: AttributeType,
{
    fn path(&self) -> DGPath {
        let path = format!("{}.{}", &self.node.path, &self.name);
        DGPath {
            path: path,
            item_type: DGITemType::Attribute,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    impl AttributeType for bool {}

    use crate::node::NodeType;
    struct NullNode {}
    impl NodeType for NullNode {
        fn evaluate<T: NodeType>(&self, node: &Node<T>){}
    }

    #[test]
    fn test_attribute_path() {
        let node = Node::new(String::from("node"), None, NullNode{});
        let attribute = Attribute::new(String::from("attribute"), node.path(), false);
        assert_eq!(attribute.path(), "|node.attribute");
    }
}
