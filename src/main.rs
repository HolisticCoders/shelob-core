mod attribute;
mod node;
mod scene;
mod add;

use scene::Scene;
use node::Node;
use add::Add;

fn main() {
    let mut scene = Scene::new();
    let node: Add = Node::new();

    scene.add_node(node);

    println!("{:#?}", scene.nodes);
    println!("{:#?}", scene.node_graph);
}
