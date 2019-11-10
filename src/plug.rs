use crate::dg::{DGITemType, DGItem, DGPath};
use crate::errors::ConnectionError;
use crate::socket::Socket;

pub struct Plug {
    pub name: String,
    pub node: DGPath,
    pub attribute: DGPath,
    pub sockets: Vec<DGPath>,
}

impl Plug {
    pub fn new(name: String, node: DGPath, attribute: DGPath) -> Self {
        Plug {
            name: name,
            node: node,
            attribute: attribute,
            sockets: vec![],
        }
    }
}

impl Plug {
    pub fn connect(&mut self, other: &mut Socket) -> Result<(), ConnectionError> {
        if self.node == other.node {
            return Err(ConnectionError::SameNode)
        }
        for socket in &self.sockets {
            if *socket == other.path() {
                return Err(ConnectionError::AlreadyConnected)
            }
        }
        self.sockets.push(other.path());
        other.plug = Some(self.path());
        Ok(())
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
    use crate::node::Node;
    #[test]
    fn test_plug_path() {
        let node = Node::new(String::from("node"), None);
        let attribute = Attribute::new(String::from("attribute"), node.path(), false);
        let plug = Plug::new(String::from("plug"), node.path(), attribute.path());
        assert_eq!(plug.path(), "|node<plug");
    }

    #[test]
    fn test_connect_plug() {
        let node1 = Node::new(String::from("node1"), None);
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path(), false);
        let mut plug = Plug::new(String::from("plug"), node1.path(), attribute1.path());

        let node2 = Node::new(String::from("node2"), None);
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path(), false);
        let mut socket = Socket::new(String::from("socket"), node2.path(), attribute2.path());

        let res = plug.connect(&mut socket);
        assert!(res.is_ok());
    }

    #[test]
    fn test_connect_plug_already_connected() {
        let node1 = Node::new(String::from("node1"), None);
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path(), false);
        let mut plug = Plug::new(String::from("plug"), node1.path(), attribute1.path());

        let node2 = Node::new(String::from("node2"), None);
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path(), false);
        let mut socket = Socket::new(String::from("socket"), node2.path(), attribute2.path());

        plug.connect(&mut socket).unwrap();

        let res = plug.connect(&mut socket);
        assert_eq!(res, Err(ConnectionError::AlreadyConnected));
    }

    #[test]
    fn test_connect_plug_same_node() {
        let node = Node::new(String::from("node"), None);
        let attribute = Attribute::new(String::from("attribute"), node.path(), false);
        let mut plug = Plug::new(String::from("plug"), node.path(), attribute.path());
        let mut socket = Socket::new(String::from("socket"), node.path(), attribute.path());

        let res = plug.connect(&mut socket);
        assert_eq!(res, Err(ConnectionError::SameNode));
    }
}
