use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy,Clone,PartialEq, Debug)]
pub(super) enum Size {
    Absolute(u16),
    Percentage(u16),
}
impl Size {
    pub(super) fn is_absolute(&self) -> bool {
        match self {
            Size::Absolute(_) => true,
            Size::Percentage(_) => false,
        }
    }
    pub(super) fn as_absolute_size(&self, parent_size: u16) -> u16 {
        match self {
            Size::Absolute(v) => {
                *v
            }
            Size::Percentage(v) => {
                (((*v) as u32) * (parent_size as u32) / 10000u32).clamp(0, 0xFFFF) as u16
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => {
                Some(Size::Absolute(value.numerical_value.clamp(0, 30000) as u16))
            }
            ValueType::Percentage => {
                Some(Size::Percentage(value.numerical_value.clamp(0, 30000) as u16))
            }
            _ => {
                None
            }
        }
    }
}
