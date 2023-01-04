use crate::utils::{KeyValuePair, ValueType};

pub (super) enum Coordonate {
    Absolute(i32),
    Percentage(i32)
}
impl Coordonate {
    pub (super) fn to_absolute_coordonate(&self, parent_size: u32)->i32 {
        match self
        {
            Coordonate::Absolute(v) => { return *v; },
            Coordonate::Percentage(v) => { return v * (parent_size as i32) / 10000; }
        }
    }
    pub (super) fn new(value: &KeyValuePair) -> Self {
        match value.value_type {
            ValueType::Number => { return Coordonate::Absolute(value.numerical_value); }
            ValueType::Percentage => { return Coordonate::Percentage(value.numerical_value); }
            _ => { return Coordonate::Absolute(0); }
        }
    }
}