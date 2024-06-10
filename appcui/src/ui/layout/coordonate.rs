#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Coordonate {
    Absolute(i32),
    Percentage(f32),
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
            Coordonate::Absolute(v) => *v,
            Coordonate::Percentage(v) => ((parent_size as f32) * v) as i32,
        }
    }

    pub fn update_with_absolute_value(&mut self, value: i16, parent_size: u16) {
        match self {
            Coordonate::Absolute(v) => {
                *v = value as i32;
            }
            Coordonate::Percentage(v) => {
                if parent_size > 0 {
                    *v = (value as f32) / (parent_size as f32);
                } else {
                    *v = 0.0f32;
                }
            }
        }
    }
}

impl From<i8> for Coordonate {
    fn from(value: i8) -> Self {
        Coordonate::Absolute(value as i32)
    }
}
impl From<i16> for Coordonate {
    fn from(value: i16) -> Self {
        Coordonate::Absolute(value as i32)
    }
}
impl From<i32> for Coordonate {
    fn from(value: i32) -> Self {
        Coordonate::Absolute(value)
    }
}
impl From<i64> for Coordonate {
    fn from(value: i64) -> Self {
        Coordonate::Absolute(value as i32)
    }
}
impl From<u8> for Coordonate {
    fn from(value: u8) -> Self {
        Coordonate::Absolute(value as i32)
    }
}
impl From<u16> for Coordonate {
    fn from(value: u16) -> Self {
        Coordonate::Absolute(value as i32)
    }
}

impl From<f32> for Coordonate {
    fn from(value: f32) -> Self {
        Coordonate::Percentage(value)
    }
}
impl From<f64> for Coordonate {
    fn from(value: f64) -> Self {
        Coordonate::Percentage(value as f32)
    }
}
