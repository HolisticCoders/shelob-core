pub trait AttributeType {}

pub struct Attribute {
    name: String,
    value: AttributeType,
}