
const MAX_DIMENSION: i32 = 30000;
const MIN_DIMENSION: i32 = 0;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
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
}
impl From<i8> for Dimension16 {
    fn from(value: i8) -> Self {
        Dimension16::Absolute(value.max(0) as u16)
    }
}
impl From<i16> for Dimension16 {
    fn from(value: i16) -> Self {
        Dimension16::Absolute(value.clamp(MIN_DIMENSION as i16, MAX_DIMENSION as i16) as u16)
    }
}
impl From<i32> for Dimension16 {
    fn from(value: i32) -> Self {
        Dimension16::Absolute(value.clamp(MIN_DIMENSION, MAX_DIMENSION) as u16)
    }
}
impl From<i64> for Dimension16 {
    fn from(value: i64) -> Self {
        Dimension16::Absolute(value.clamp(MIN_DIMENSION as i64, MAX_DIMENSION as i64) as u16)
    }
}
impl From<u8> for Dimension16 {
    fn from(value: u8) -> Self {
        Dimension16::Absolute(value as u16)
    }
}
impl From<u16> for Dimension16 {
    fn from(value: u16) -> Self {
        Dimension16::Absolute(value.min(MAX_DIMENSION as u16) as u16)
    }
}
impl From<u32> for Dimension16 {
    fn from(value: u32) -> Self {
        Dimension16::Absolute(value.min(MAX_DIMENSION as u32) as u16)
    }
}
impl From<u64> for Dimension16 {
    fn from(value: u64) -> Self {
        Dimension16::Absolute(value.min(MAX_DIMENSION as u64) as u16)
    }
}
impl From<f64> for Dimension16 {
    fn from(value: f64) -> Self {
        Dimension16::Percentage(((value * 10000.0f64) as i32).clamp(MIN_DIMENSION, MAX_DIMENSION) as u16)
    }
}
impl From<f32> for Dimension16 {
    fn from(value: f32) -> Self {
        Dimension16::Percentage(((value * 10000.0f32) as i32).clamp(MIN_DIMENSION, MAX_DIMENSION) as u16)
    }
}