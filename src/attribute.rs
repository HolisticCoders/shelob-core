use crate::dg::{DGITemType, DGItem, DGPath};

pub struct Attribute {
    pub name: String,
    pub node: DGPath,
}

impl Attribute {
    pub fn new(name: String, node: DGPath) -> Self {
        Attribute {
            name: name,
            node: node,
        }
    }
}

impl DGItem for Attribute {
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
    #[test]
    fn test_attribute_path() {
        let node = Node::new(String::from("node"), None);
        let attribute = Attribute::new(String::from("attribute"), node.path());
        assert_eq!(attribute.path(), "|node.attribute");
    }
}
