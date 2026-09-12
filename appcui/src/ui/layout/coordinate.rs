#[derive(Copy, Clone, PartialEq, Debug)]
/// A position along one axis, either in character cells or as a fraction of the parent.
///
/// Absolute values are in cells. Percentage values are typically between `0.0` and
/// `1.0` of the parent size.
pub enum Coordinate {
    /// Offset in character cells, for example `10`.
    Absolute(i32),
    /// Fraction of the parent size, typically `0.0`–`1.0` (for example `0.5` is 50%).
    Percentage(f32),
}
impl Coordinate {
    pub fn is_absolute(&self) -> bool {
        match self {
            Coordinate::Absolute(_) => true,
            Coordinate::Percentage(_) => false,
        }
    }
    pub fn absolute(&self, parent_size: u16) -> i32 {
        match self {
            Coordinate::Absolute(v) => *v,
            Coordinate::Percentage(v) => ((parent_size as f32) * v) as i32,
        }
    }

    pub fn update_with_absolute_value(&mut self, value: i16, parent_size: u16) {
        match self {
            Coordinate::Absolute(v) => {
                *v = value as i32;
            }
            Coordinate::Percentage(v) => {
                if parent_size > 0 {
                    *v = (value as f32) / (parent_size as f32);
                } else {
                    *v = 0.0f32;
                }
            }
        }
    }
}

impl From<i8> for Coordinate {
    fn from(value: i8) -> Self {
        Coordinate::Absolute(value as i32)
    }
}
impl From<i16> for Coordinate {
    fn from(value: i16) -> Self {
        Coordinate::Absolute(value as i32)
    }
}
impl From<i32> for Coordinate {
    fn from(value: i32) -> Self {
        Coordinate::Absolute(value)
    }
}
impl From<i64> for Coordinate {
    fn from(value: i64) -> Self {
        Coordinate::Absolute(value as i32)
    }
}
impl From<u8> for Coordinate {
    fn from(value: u8) -> Self {
        Coordinate::Absolute(value as i32)
    }
}
impl From<u16> for Coordinate {
    fn from(value: u16) -> Self {
        Coordinate::Absolute(value as i32)
    }
}

impl From<f32> for Coordinate {
    fn from(value: f32) -> Self {
        Coordinate::Percentage(value)
    }
}
impl From<f64> for Coordinate {
    fn from(value: f64) -> Self {
        Coordinate::Percentage(value as f32)
    }
}
