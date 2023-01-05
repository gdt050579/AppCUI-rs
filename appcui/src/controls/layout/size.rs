use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy,Clone)]
pub(super) enum Size {
    Absolute(u16),
    Percentage(u16),
}
impl Size {
    pub(super) fn to_absolute_coordonate(&self, parent_size: u32) -> u32 {
        match self {
            Size::Absolute(v) => {
                return (*v) as u32;
            }
            Size::Percentage(v) => {
                return ((*v) as u32) * parent_size / 10000u32;
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Self {
        match value.value_type {
            ValueType::Number => {
                return Size::Absolute(value.numerical_value.clamp(0, 30000) as u16);
            }
            ValueType::Percentage => {
                return Size::Percentage(value.numerical_value.clamp(0, 30000) as u16);
            }
            _ => {
                return Size::Absolute(0);
            }
        }
    }
}
