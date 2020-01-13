use crate::attribute::Attribute;
use crate::node::Node;

#[derive(Debug)]
pub struct Add {
    a: Attribute,
    b: Attribute,
}

impl Node for Add {
    fn new() -> Self {
        Add {
            a: Attribute::new(),
            b: Attribute::new(),
        }
    }

    fn evaluate(&self) {
        println!("Evaluating!")
    }

    fn attributes(&self) -> Vec<&Attribute> {
        vec![&self.a, &self.b]
    }
}
