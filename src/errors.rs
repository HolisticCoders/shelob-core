#[derive(Debug, PartialEq)]
pub enum SceneError {
    ItemAlreadyExists,
}

#[derive(Debug, PartialEq)]
pub enum ConnectionError {
    AlreadyConnected,
    SameNode,
}

