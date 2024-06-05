use crate::utils::{KeyValuePair, ValueType};

#[derive(Copy,Clone,PartialEq, Debug)]
pub enum Dimension {
    Absolute(u16),
    Percentage(u16),
}
impl Dimension {
    pub fn is_absolute(&self) -> bool {
        match self {
            Dimension::Absolute(_) => true,
            Dimension::Percentage(_) => false,
        }
    }
    pub fn absolute_size(&self, parent_size: u16) -> u16 {
        match self {
            Dimension::Absolute(v) => {
                *v
            }
            Dimension::Percentage(v) => {
                (((*v) as u32) * (parent_size as u32) / 10000u32).clamp(0, 0xFFFF) as u16
            }
        }
    }
    pub(super) fn new(value: &KeyValuePair) -> Option<Self> {
        match value.value_type {
            ValueType::Number => {
                Some(Dimension::Absolute(value.numerical_value.clamp(0, 30000) as u16))
            }
            ValueType::Percentage => {
                Some(Dimension::Percentage(value.numerical_value.clamp(0, 30000) as u16))
            }
            _ => {
                None
            }
        }
    }
}
impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Dimension::Absolute(value)
    }
}
impl From<u8> for Dimension {
    fn from(value: u8) -> Self {
        Dimension::Absolute(value as u16)
    }
}
impl From<u32> for Dimension {
    fn from(value: u32) -> Self {
        Dimension::Absolute(value as u16)
    }
}
impl From<u64> for Dimension {
    fn from(value: u64) -> Self {
        Dimension::Absolute(value as u16)
    }
}
impl From<i8> for Dimension {
    fn from(value: i8) -> Self {
        if value>0 {
            Dimension::Absolute(value as u16)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i16> for Dimension {
    fn from(value: i16) -> Self {
        if value>0 {
            Dimension::Absolute(value as u16)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i32> for Dimension {
    fn from(value: i32) -> Self {
        if value>0 {
            Dimension::Absolute(value as u16)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i64> for Dimension {
    fn from(value: i64) -> Self {
        if value>0 {
            Dimension::Absolute(value as u16)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<f32> for Dimension {
    fn from(value: f32) -> Self {
        if value<0.0 {
            Dimension::Percentage(0)
        } else {
            Dimension::Percentage((value * 10000.0f32) as u16)
        }
    }
}
impl From<f64> for Dimension {
    fn from(value: f64) -> Self {
        if value<0.0 {
            Dimension::Percentage(0)
        } else {
            Dimension::Percentage((value * 10000.0f64) as u16)
        }
    }
}