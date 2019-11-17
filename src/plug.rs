use crate::dg::{DGITemType, DGItem, DGPath};
use crate::attribute::AttributeType;
use crate::errors::ConnectionError;

#[derive(PartialEq, Eq)]
pub enum PlugType {
    Input,
    Output,
    Both,
}

pub struct Plug {
    pub name: String,
    pub node: DGPath,
    pub attribute: DGPath,
    pub plug_type: PlugType,
    pub input: Option<DGPath>,
    pub outputs: Vec<DGPath>,
}

impl Plug {
    pub fn new(name: String, node: DGPath, attribute: DGPath, plug_type: PlugType) -> Self {
        Plug {
            name: name,
            node: node,
            attribute: attribute,
            plug_type: plug_type,
            input: None,
            outputs: vec![],
        }
    }
}

impl Plug {
    pub fn connect(&mut self, target: &mut Plug) -> Result<(), ConnectionError> {
        if self.node == target.node {
            return Err(ConnectionError::SameNode);
        }

        if self.plug_type == PlugType::Input {
            return Err(ConnectionError::SourceIsInputOnly);
        }

        if target.plug_type == PlugType::Output {
            return Err(ConnectionError::TargetIsOutputOnly);
        }

        for output in &self.outputs {
            if *output == target.path() {
                return Err(ConnectionError::AlreadyConnected);
            }
        }

        self.outputs.push(target.path());
        target.input = Some(self.path());
        Ok(())
    }

    pub fn set<T: AttributeType>(&self, value: T) {
        // self.attribute.value = value
    }
    pub fn get(&self) {
        // self.attribute.value
    }
}

impl DGItem for Plug {
    fn path(&self) -> DGPath {
        let path = format!("{}<{}", &self.node.path, &self.name);
        DGPath {
            path: path,
            item_type: DGITemType::Plug,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::Attribute;
    use crate::node::{Node, NodeType};
    struct NullNode {}
    impl NodeType for NullNode {
        fn evaluate<T: NodeType>(&self, node: &Node<T>) {}
    }

    #[test]
    fn test_plug_path() {
        let node = Node::new(String::from("node"), None, NullNode {});
        let attribute = Attribute::new(String::from("attribute"), node.path(), false);
        let plug = Plug::new(String::from("plug"), node.path(), attribute.path(), PlugType::Input);
        assert_eq!(plug.path(), "|node<plug");
    }

    #[test]
    fn test_connect_plug() {
        let node1 = Node::new(String::from("node1"), None, NullNode {});
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path(), false);
        let mut output = Plug::new(String::from("output"), node1.path(), attribute1.path(), PlugType::Output);

        let node2 = Node::new(String::from("node2"), None, NullNode {});
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path(), false);
        let mut input = Plug::new(String::from("input"), node2.path(), attribute2.path(), PlugType::Input);

        let res = output.connect(&mut input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_connect_plug_already_connected() {
        let node1 = Node::new(String::from("node1"), None, NullNode {});
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path(), false);
        let mut output = Plug::new(String::from("output"), node1.path(), attribute1.path(), PlugType::Output);

        let node2 = Node::new(String::from("node2"), None, NullNode {});
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path(), false);
        let mut input = Plug::new(String::from("input"), node2.path(), attribute2.path(), PlugType::Input);

        output.connect(&mut input).unwrap();

        let res = output.connect(&mut input);
        assert_eq!(res, Err(ConnectionError::AlreadyConnected));
    }

    #[test]
    fn test_connect_plug_same_node() {
        let node = Node::new(String::from("node"), None, NullNode {});
        let attribute = Attribute::new(String::from("attribute"), node.path(), false);
        let mut output = Plug::new(String::from("output"), node.path(), attribute.path(), PlugType::Output);
        let mut input = Plug::new(String::from("input"), node.path(), attribute.path(), PlugType::Input);

        let res = output.connect(&mut input);
        assert_eq!(res, Err(ConnectionError::SameNode));
    }
}
