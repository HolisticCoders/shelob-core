use crate::dg::{DGITemType, DGItem, DGPath};
use crate::attribute::AttributeType;

pub trait NodeType {
    fn evaluate<T: NodeType>(&self, node: &Node<T>);
}

pub struct Node<T>
where
    T: NodeType,
{
    pub name: String,
    pub parent: Option<DGPath>,
    pub attributes: Vec<DGPath>,
    pub plugs: Vec<DGPath>,
    pub sockets: Vec<DGPath>,
    nodetype: T,
}

impl<T> Node<T>
where
    T: NodeType,
{
    pub fn new(name: String, parent: Option<DGPath>, nodetype: T) -> Self {
        Node {
            name: name,
            parent: parent,
            nodetype: nodetype,
            attributes: vec![],
            plugs: vec![],
            sockets: vec![],
        }
    }
}

impl<T> Node<T>
where
    T: NodeType,
{
    pub fn evaluate(&self){
        self.nodetype.evaluate(&self);
    }
    pub fn add_attribute<A: AttributeType>(&self, name: String, value: A){}
    pub fn delete_attribute<A: AttributeType>(&self, name: String, value: A) {}
    pub fn add_plug(&self, name: String) {}
    pub fn delete_plug(&self, name: String) {}
    pub fn add_socket(&self, name: String) {}
    pub fn delete_socket(&self, name: String) {}
}

impl<T> DGItem for Node<T>
where
    T: NodeType,
{
    fn path(&self) -> DGPath {
        let path = match &self.parent {
            None => format!("|{}", self.name.clone()),
            Some(parent) => format!("{}|{}", parent.path, &self.name),
        };
        DGPath {
            path: path,
            item_type: DGITemType::Node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct NullNode {}
    impl NodeType for NullNode {
        fn evaluate<T: NodeType>(&self, node: &Node<T>){}
    }
    #[test]
    fn test_parent() {
        let parent = Node::new(String::from("parent"), None, NullNode{});
        let child = Node::new(String::from("child"), Some(parent.path()), NullNode{});
        assert_eq!(child.path(), "|parent|child");
    }
}
