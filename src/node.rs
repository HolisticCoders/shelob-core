use crate::dg::{DGITemType, DGItem, DGPath};

pub struct Node {
    pub name: String,
    pub parent: Option<DGPath>,
}

impl Node {
    pub fn new(name: String, parent: Option<DGPath>) -> Self {
        Node {
            name: name,
            parent: parent,
        }
    }
}

impl DGItem for Node {
    fn path(&self) -> DGPath {
        let path = match &self.parent {
            None => format!("|{}", self.name.clone()),
            Some(parent) => format!("{}|{}", parent.path, &self.name),
        };
        DGPath {
            path: path,
            item_type: DGITemType::Node
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parent() {
        let parent = Node::new(String::from("parent"), None);
        let child = Node::new(String::from("child"), Some(parent.path()));
        assert_eq!(child.path(), "|parent|child");
    }
}
