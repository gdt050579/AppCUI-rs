use crate::utils::{KeyValuePair, ValueType};

pub (super) enum Size {
    Absolute(u32),
    Percentage(u32)
}
impl Size {
    pub (super) fn to_absolute_coordonate(&self, parent_size: u32)->u32 {
        match self
        {
            Size::Absolute(v) => { return *v; },
            Size::Percentage(v) => { return v * parent_size  / 10000u32; }
        }
    }
    pub (super) fn new(value: &KeyValuePair) -> Self {
        match value.value_type {
            ValueType::Number => { return Size::Absolute(value.numerical_value as u32); }
            ValueType::Percentage => { return Size::Percentage(value.numerical_value as u32); }
            _ => { return Size::Absolute(0); }
        }
    }    
}