#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Absolute(u32),
    Percentage(f32),
}
impl Dimension {
    pub fn is_absolute(&self) -> bool {
        match self {
            Dimension::Absolute(_) => true,
            Dimension::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> u16 {
        match self {
            Dimension::Absolute(v) => (*v) as u16,
            Dimension::Percentage(v) =>((parent_size as f32) * v) as u16,
        }
    }
}
impl From<u16> for Dimension {
    fn from(value: u16) -> Self {
        Dimension::Absolute(value as u32)
    }
}
impl From<u8> for Dimension {
    fn from(value: u8) -> Self {
        Dimension::Absolute(value as u32)
    }
}
impl From<u32> for Dimension {
    fn from(value: u32) -> Self {
        Dimension::Absolute(value)
    }
}
impl From<u64> for Dimension {
    fn from(value: u64) -> Self {
        Dimension::Absolute(value as u32)
    }
}
impl From<i8> for Dimension {
    fn from(value: i8) -> Self {
        if value > 0 {
            Dimension::Absolute(value as u32)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i16> for Dimension {
    fn from(value: i16) -> Self {
        if value > 0 {
            Dimension::Absolute(value as u32)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i32> for Dimension {
    fn from(value: i32) -> Self {
        if value > 0 {
            Dimension::Absolute(value as u32)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<i64> for Dimension {
    fn from(value: i64) -> Self {
        if value > 0 {
            Dimension::Absolute(value as u32)
        } else {
            Dimension::Absolute(0)
        }
    }
}
impl From<f32> for Dimension {
    fn from(value: f32) -> Self {
        Dimension::Percentage(value)
    }
}
impl From<f64> for Dimension {
    fn from(value: f64) -> Self {
        Dimension::Percentage(value as f32)
    }
}
