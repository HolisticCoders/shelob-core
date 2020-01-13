
use std::collections::HashMap;

use petgraph::Graph;
use uuid::Uuid;

use crate::node::Node;
use crate::attribute::Attribute;

pub struct Scene {
    pub nodes: HashMap<Uuid, Node>,
    pub attributes: HashMap<Uuid, Attribute>,

    pub node_graph: Graph<Uuid, Uuid>,
    pub attribute_graph: Graph<Uuid, Uuid>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            nodes: HashMap::new(),
            attributes: HashMap::new(),
            node_graph: Graph::new(),
            attribute_graph: Graph::new(),
        }
    }

    pub fn add_node(&mut self) {
        let node = Node::new();
        self.node_graph.add_node(node.uuid);
        self.nodes.insert(node.uuid, node);
    }
}
