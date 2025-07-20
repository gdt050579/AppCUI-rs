use crate::utils::{KeyValuePair, ValueType};

const MAX_COORDONATE: i32 = 30000;
const MIN_COORDONATE: i32 = -30000;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Coordinate16 {
    Absolute(i16),
    Percentage(i16),
}
impl Coordinate16 {
    pub fn is_absolute(&self) -> bool {
        match self {
            Coordinate16::Absolute(_) => true,
            Coordinate16::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> i32 {
        match self {
            Coordinate16::Absolute(v) => (*v) as i32,
            Coordinate16::Percentage(v) => ((*v) as i32) * (parent_size as i32) / 10000i32,
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => Some(Coordinate16::Absolute(value.numerical_value.clamp(MIN_COORDONATE, MAX_COORDONATE) as i16)),
            ValueType::Percentage => Some(Coordinate16::Percentage(value.numerical_value.clamp(MIN_COORDONATE, MAX_COORDONATE) as i16)),
            _ => None,
        }
    }
}

impl From<i8> for Coordinate16 {
    fn from(value: i8) -> Self {
        Coordinate16::Absolute(value as i16)
    }
}
impl From<i16> for Coordinate16 {
    fn from(value: i16) -> Self {
        Coordinate16::Absolute(value.clamp(MIN_COORDONATE as i16, MAX_COORDONATE as i16))
    }
}
impl From<i32> for Coordinate16 {
    fn from(value: i32) -> Self {
        Coordinate16::Absolute(value.clamp(MIN_COORDONATE, MAX_COORDONATE) as i16)
    }
}
impl From<i64> for Coordinate16 {
    fn from(value: i64) -> Self {
        Coordinate16::Absolute(value.clamp(MIN_COORDONATE as i64, MAX_COORDONATE as i64) as i16)
    }
}
impl From<u8> for Coordinate16 {
    fn from(value: u8) -> Self {
        Coordinate16::Absolute(value as i16)
    }
}
impl From<u16> for Coordinate16 {
    fn from(value: u16) -> Self {
        Coordinate16::Absolute(value.min(MAX_COORDONATE as u16) as i16)
    }
}
impl From<u32> for Coordinate16 {
    fn from(value: u32) -> Self {
        Coordinate16::Absolute(value.min(MAX_COORDONATE as u32) as i16)
    }
}
impl From<u64> for Coordinate16 {
    fn from(value: u64) -> Self {
        Coordinate16::Absolute(value.min(MAX_COORDONATE as u64) as i16)
    }
}
impl From<f32> for Coordinate16 {
    fn from(value: f32) -> Self {
        Coordinate16::Percentage(((value * 10000.0f32) as i32).clamp(MIN_COORDONATE, MAX_COORDONATE) as i16)
    }
}

impl From<f64> for Coordinate16 {
    fn from(value: f64) -> Self {
        Coordinate16::Percentage(((value * 10000.0f64) as i32).clamp(MIN_COORDONATE, MAX_COORDONATE) as i16)
    }
}
