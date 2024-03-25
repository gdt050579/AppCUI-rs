use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) enum Coordonate {
    Absolute(i16),
    Percentage(i16),
}
impl Coordonate {
    pub(super) fn is_absolute(&self) -> bool {
        match self {
            Coordonate::Absolute(_) => true,
            Coordonate::Percentage(_) => false,
        }
    }
    pub(super) fn as_absolute_coordonate(&self, parent_size: u16) -> i32 {
        match self {
            Coordonate::Absolute(v) => {
                (*v) as i32
            }
            Coordonate::Percentage(v) => {
                ((*v) as i32) * (parent_size as i32) / 10000i32
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => {
                Some(Coordonate::Absolute(
                    value.numerical_value.clamp(-30000, 30000) as i16,
                ))
            }
            ValueType::Percentage => {
                Some(Coordonate::Percentage(
                    value.numerical_value.clamp(-30000, 30000) as i16,
                ))
            }
            _ => {
                None
            }
        }
    }
}
