use crate::utils::{KeyValuePair, ValueType};

pub(super) enum Coordonate {
    Absolute(i16),
    Percentage(i16),
}
impl Coordonate {
    pub(super) fn to_absolute_coordonate(&self, parent_size: u32) -> i32 {
        match self {
            Coordonate::Absolute(v) => {
                return (*v) as i32;
            }
            Coordonate::Percentage(v) => {
                return ((*v) as i32) * (parent_size as i32) / 10000i32;
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Self {
        match value.value_type {
            ValueType::Number => {
                return Coordonate::Absolute(value.numerical_value.clamp(-30000, 30000) as i16);
            }
            ValueType::Percentage => {
                return Coordonate::Percentage(value.numerical_value.clamp(-30000, 30000) as i16);
            }
            _ => {
                return Coordonate::Absolute(0);
            }
        }
    }
}
