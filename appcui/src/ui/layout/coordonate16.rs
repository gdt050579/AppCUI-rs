use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Coordonate16 {
    Absolute(i16),
    Percentage(i16),
}
impl Coordonate16 {
    pub fn is_absolute(&self) -> bool {
        match self {
            Coordonate16::Absolute(_) => true,
            Coordonate16::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> i32 {
        match self {
            Coordonate16::Absolute(v) => (*v) as i32,
            Coordonate16::Percentage(v) => ((*v) as i32) * (parent_size as i32) / 10000i32,
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => Some(Coordonate16::Absolute(value.numerical_value.clamp(-30000, 30000) as i16)),
            ValueType::Percentage => Some(Coordonate16::Percentage(value.numerical_value.clamp(-30000, 30000) as i16)),
            _ => None,
        }
    }
}
