use std::collections::HashMap;

use petgraph::Graph;
use uuid::Uuid;

use crate::attribute::Attribute;

pub struct Scene<T> {
    pub nodes: HashMap<Uuid, T>,
    pub attributes: HashMap<Uuid, Attribute>,

    pub node_graph: Graph<Uuid, Uuid>,
    pub attribute_graph: Graph<Uuid, Uuid>,
}

impl<T> Scene<T> {
    pub fn new() -> Self {
        Scene {
            nodes: HashMap::new(),
            attributes: HashMap::new(),
            node_graph: Graph::new(),
            attribute_graph: Graph::new(),
        }
    }

    pub fn add_node(&mut self, node: T) {
        let node_uuid = Uuid::new_v4();
        self.node_graph.add_node(node_uuid);
        self.nodes.insert(node_uuid, node);
    }
}
