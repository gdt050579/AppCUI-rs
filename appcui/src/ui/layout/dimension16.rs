use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension16 {
    Absolute(u16),
    Percentage(u16),
}
impl Dimension16 {
    pub fn is_absolute(&self) -> bool {
        match self {
            Dimension16::Absolute(_) => true,
            Dimension16::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> u16 {
        match self {
            Dimension16::Absolute(v) => *v,
            Dimension16::Percentage(v) => (((*v) as u32) * (parent_size as u32) / 10000u32).clamp(0, 0xFFFF) as u16,
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => Some(Dimension16::Absolute(value.numerical_value.clamp(0, 30000) as u16)),
            ValueType::Percentage => Some(Dimension16::Percentage(value.numerical_value.clamp(0, 30000) as u16)),
            _ => None,
        }
    }
}
