use crate::dg::{DGITemType, DGItem, DGPath};
use crate::errors::ConnectionError;
use crate::plug::Plug;

pub struct Socket {
    pub name: String,
    pub node: DGPath,
    pub attribute: DGPath,
    pub plug: Option<DGPath>,
}

impl Socket {
    pub fn new(name: String, node: DGPath, attribute: DGPath) -> Self {
        Socket {
            name: name,
            node: node,
            attribute: attribute,
            plug: None,
        }
    }
}

impl Socket {
    pub fn connect(&mut self, other: &mut Plug) -> Result<(), ConnectionError> {
        if self.node == other.node {
            return Err(ConnectionError::SameNode)
        }
        if self.plug == Some(other.path()) {
            return Err(ConnectionError::AlreadyConnected)
        }

        self.plug = Some(other.path());
        other.sockets.push(self.path());
        Ok(())
    }
}

impl DGItem for Socket {
    fn path(&self) -> DGPath {
        let path = format!("{}>{}", &self.node.path, &self.name);
        DGPath {
            path: path,
            item_type: DGITemType::Socket,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::Attribute;
    use crate::node::Node;
    #[test]
    fn test_socket_path() {
        let node = Node::new(String::from("node"), None);
        let attribute = Attribute::new(String::from("attribute"), node.path());
        let socket = Socket::new(String::from("socket"), node.path(), attribute.path());
        assert_eq!(socket.path(), "|node>socket");
    }
    #[test]
    fn test_connect_socket() {
        let node1 = Node::new(String::from("node1"), None);
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path());
        let mut plug = Plug::new(String::from("plug"), node1.path(), attribute1.path());

        let node2 = Node::new(String::from("node2"), None);
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path());
        let mut socket = Socket::new(String::from("socket"), node2.path(), attribute2.path());

        let res = socket.connect(&mut plug);
        assert!(res.is_ok());
    }

    #[test]
    fn test_connect_plug_already_connected() {
        let node1 = Node::new(String::from("node1"), None);
        let attribute1 = Attribute::new(String::from("attribute1"), node1.path());
        let mut plug = Plug::new(String::from("plug"), node1.path(), attribute1.path());

        let node2 = Node::new(String::from("node2"), None);
        let attribute2 = Attribute::new(String::from("attribute2"), node2.path());
        let mut socket = Socket::new(String::from("socket"), node2.path(), attribute2.path());

        socket.connect(&mut plug).unwrap();

        let res = socket.connect(&mut plug);
        assert_eq!(res, Err(ConnectionError::AlreadyConnected));
    }

    #[test]
    fn test_connect_plug_same_node() {
        let node = Node::new(String::from("node"), None);
        let attribute = Attribute::new(String::from("attribute"), node.path());
        let mut plug = Plug::new(String::from("plug"), node.path(), attribute.path());
        let mut socket = Socket::new(String::from("socket"), node.path(), attribute.path());

        let res = socket.connect(&mut plug);
        assert_eq!(res, Err(ConnectionError::SameNode));
    }
}
