use crate::attribute::{Attribute, AttributeType};

pub enum ConnectionError {
    InvalidOrder,
    BothInputs,
    BothOutputs,
    AlreadyConnected,
    NotConnected,
    Cycle,
}

pub struct Plug<'a, T>
where
    T: AttributeType,
{
    is_input: bool,
    is_output: bool,

    input: Option<&'a Plug<'a, T>>,
    outputs: Vec<&'a Plug<'a, T>>, // should be a hash-map with uuids
    attribute: &'a Attribute<T>,
}

// related functions
impl<'a, T> Plug<'a, T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn new(is_input: bool, is_output: bool, attribute: &'a Attribute<T>) -> Self {
        Plug {
            is_input: is_input,
            is_output: is_output,
            input: None,
            outputs: vec![],
            attribute: attribute,
        }
    }
}

// methods
impl<'a, T> Plug<'a, T>
where
    T: AttributeType,
{
    #[allow(dead_code)]
    pub fn get(&self) -> &T {
        self.attribute.get()
    }

    // FIXME: not working
    // pub fn set(&mut self, value: T) {
    //     self.attribute.set(value)
    // }

    // TODO: actually implement this
    pub fn check_cycle(&self, other: &Plug<T>) -> bool {
        false
    }

    #[allow(dead_code)]
    pub fn connect(&'a mut self, other: &'a mut Plug<'a, T>) -> Result<(), ConnectionError> {
        let already_connected = match other.input {
            Some(_) => true,
            None => false,
        };
        let invalid_order = self.is_input && other.is_output;
        let both_inputs = self.is_input && other.is_input;
        let both_outputs = self.is_output && other.is_output;
        let cycle = self.check_cycle(other);

        if already_connected {
            return Err(ConnectionError::AlreadyConnected);
        } else if invalid_order {
            return Err(ConnectionError::InvalidOrder);
        } else if both_inputs {
            return Err(ConnectionError::BothInputs);
        } else if both_outputs {
            return Err(ConnectionError::BothOutputs);
        } else if cycle {
            return Err(ConnectionError::Cycle);
        }

        // self.outputs.push(other);
        // other.input = Some(self); // FIXME: can't assign to immutable ref

        Ok(())
    }
    #[allow(dead_code)]
    pub fn disconnect(&mut self, other: &'a mut Plug<'a, T>) -> Result<(), ConnectionError> {
        let not_connected = match other.input {
            None => true,
            // TODO: Implement comparison operators or check that the pointer is the same?
            // Some(plug) => if plug == self {true} else {false},
            Some(_) => false,
        };
        if not_connected {
            return Err(ConnectionError::NotConnected);
        }

        // TODO: outputs should be a hashmap
        // self.outputs.remove(other); // This is dummy code
        // other.input = None;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_boolean_attribute() -> Attribute<bool> {
        Attribute::new("attribute".to_string(), true)
    }

    #[test]
    fn create_plug() {
        let attribute = create_boolean_attribute();
        Plug::new(true, false, &attribute);
    }
    fn connect_plugs() {
        let attribute1 = create_boolean_attribute();
        let attribute2 = create_boolean_attribute();
        let mut plug1 = Plug::new(true, false, &attribute1);
        let mut plug2 = Plug::new(true, false, &attribute2);
        plug1.connect(&mut plug2);
    }
}
