use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Coordonate {
    Absolute(i16),
    Percentage(i16),
}
impl Coordonate {
    pub fn is_absolute(&self) -> bool {
        match self {
            Coordonate::Absolute(_) => true,
            Coordonate::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> i32 {
        match self {
            Coordonate::Absolute(v) => (*v) as i32,
            Coordonate::Percentage(v) => ((*v) as i32) * (parent_size as i32) / 10000i32,
        }
    }
    
    pub fn update_with_absolute_value(&mut self, value: i16, parent_size: u16) {
        match self {
            Coordonate::Absolute(v) => {
                *v = value;
            }
            Coordonate::Percentage(v) => {
                if parent_size > 0 {
                    *v = ((value as i32) * 10000 / (parent_size as i32)) as i16;
                } else {
                    *v = 0;
                }
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => Some(Coordonate::Absolute(value.numerical_value.clamp(-30000, 30000) as i16)),
            ValueType::Percentage => Some(Coordonate::Percentage(value.numerical_value.clamp(-30000, 30000) as i16)),
            _ => None,
        }
    }
}

impl From<i8> for Coordonate {
    fn from(value: i8) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<i16> for Coordonate {
    fn from(value: i16) -> Self {
        Coordonate::Absolute(value)
    }
}
impl From<i32> for Coordonate {
    fn from(value: i32) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<i64> for Coordonate {
    fn from(value: i64) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<u8> for Coordonate {
    fn from(value: u8) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<u16> for Coordonate {
    fn from(value: u16) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<u32> for Coordonate {
    fn from(value: u32) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<u64> for Coordonate {
    fn from(value: u64) -> Self {
        Coordonate::Absolute(value as i16)
    }
}
impl From<f32> for Coordonate {
    fn from(value: f32) -> Self {
        Coordonate::Percentage((value * 10000.0f32) as i16)
    }
}
impl From<f64> for Coordonate {
    fn from(value: f64) -> Self {
        Coordonate::Percentage((value * 10000.0f64) as i16)
    }
}
