use uuid::Uuid;

#[derive(Debug)]
pub struct Attribute {
    pub uuid: Uuid,
}

impl Attribute {
    pub fn new() -> Self {
        Attribute {
            uuid: Uuid::new_v4()
        }
    }
}