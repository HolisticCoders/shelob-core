use crate::attribute::AttributeType;

struct InputPlug<'a, T>
where
    T: AttributeType,
{
    outputs: Vec<&'a OutputPlug<'a, T>>,
    attribute: &'a mut T,
}

struct OutputPlug<'a, T>
where
    T: AttributeType,
{
    input: Option<&'a InputPlug<'a, T>>,
    attribute: &'a T,
}

impl<'a, T> InputPlug<'a, T>
where
    T: AttributeType,
{
    fn connect(&mut self, other: &'a mut OutputPlug<'a, T>) {
        self.outputs.push(other);
        other.connect(self);
    }
}

impl<'a, T> OutputPlug<'a, T>
where
    T: AttributeType,
{
    fn connect(&mut self, other: &'a mut InputPlug<'_, T>) {
        self.input = Some(other);
        other.connect(self);
    }
}
