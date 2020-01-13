mod attribute;
mod node;
mod scene;

use scene::Scene;

fn main() {
    let mut scene = Scene::new();
    scene.add_node();

    println!("{:#?}", scene.nodes);
    println!("{:#?}", scene.node_graph);
}
