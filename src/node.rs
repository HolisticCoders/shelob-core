use uuid::Uuid;

#[derive(Debug)]
pub struct Node {
    pub uuid: Uuid,
}

impl Node {
    pub fn new() -> Self {
        Node {
            uuid: Uuid::new_v4()
        }
    }
}