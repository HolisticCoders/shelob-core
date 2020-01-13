use crate::attribute::Attribute;


pub trait Node {
    fn new() -> Self;
    fn evaluate(&self);
    fn attributes(&self) -> Vec<&Attribute>;
}
