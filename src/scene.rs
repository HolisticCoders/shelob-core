use std::collections::HashMap;

use crate::dg::{DGItem, DGPath};
use crate::errors::SceneError;

pub struct Scene<T>
where
    T: DGItem,
{
    items: HashMap<DGPath, T>,
}

impl<T> Scene<T>
where
    T: DGItem,
{
    pub fn new() -> Self {
        Scene {
            items: HashMap::new(),
        }
    }
}

impl<T> Scene<T>
where
    T: DGItem,
{
    pub fn add_item(&mut self, item: T) -> Result<(), SceneError> {
        match self.items.get(&item.path()) {
            Some(_) => Err(SceneError::ItemAlreadyExists),
            None => {
                self.items.insert(item.path(), item);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::node::{Node, NodeType};
    struct NullNode {}
    impl NodeType for NullNode {
        fn evaluate<T: NodeType>(&self, node: &Node<T>){}
    }


    #[test]
    fn test_add_node() {
        let mut scene = Scene::new();
        let node = Node::new(String::from("tamere"), None, NullNode{});
        let res = scene.add_item(node);
        assert!(res.is_ok());
    }
    #[test]
    fn test_add_duplicate_node() {
        let mut scene = Scene::new();

        let node = Node::new(String::from("tamere"), None, NullNode{});
        scene.add_item(node).unwrap();

        let node = Node::new(String::from("tamere"), None, NullNode{});
        let res = scene.add_item(node);
        assert_eq!(res, Err(SceneError::ItemAlreadyExists));
    }
}
