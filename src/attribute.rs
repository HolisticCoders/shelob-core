pub trait AttributeType {}

pub struct Attribute<T>
where
    T: AttributeType,
{
    pub name: String,
    value: T,
}

// related functions
impl<T> Attribute<T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn new(name: String, value: T) -> Self {
        Attribute {
            name: name,
            value: value,
        }
    }
}

// methods
impl<T> Attribute<T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn get(&self) -> &T {
        &self.value
    }

    #[allow(dead_code)]
    pub fn set(&mut self, value: T) {
        self.value = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl AttributeType for bool {}

    #[test]
    fn create_attribute() {
        Attribute::new("attribute".to_string(), true);
    }

    #[test]
    fn set_attribute() {
        let mut attribute = Attribute::new("attribute".to_string(), true);
        attribute.set(false);
        assert_eq!(attribute.value, false);
    }

    #[test]
    fn get_attribute() {
        let mut attribute = Attribute::new("attribute".to_string(), true);
        attribute.set(false);
        assert_eq!(attribute.get(), &false);
    }
}
