use std::{fmt::Display, str::FromStr};

/// Represents a size (width and height) of a rectangle.
#[derive(Copy, Clone, PartialEq, Debug, Eq, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Creates a new `Size` with the specified width and height.
    pub fn new(width: u32, height: u32) -> Size {
        Size { width, height }
    }
    /// Reduces the size with a specified value. If that value is greater than the width or height, that parameter will be clamped to 0.
    /// # Example
    /// ```
    /// use appcui::graphics::Size;
    /// let size = Size::new(10, 10);
    /// // Reduce the size by 3
    /// let reduced_size = size.reduce_by(3);
    /// assert_eq!(reduced_size.width, 7);
    /// assert_eq!(reduced_size.height, 7);
    /// 
    /// // Reduce the size by 100 (since 100 is greater than
    /// // the width and height, the result will be 0)
    /// let reduced_size_zero = size.reduce_by(100);
    /// assert_eq!(reduced_size_zero.width, 0);
    /// assert_eq!(reduced_size_zero.height, 0);
    /// ```
    pub fn reduce_by(&self, value: u32) -> Size {
        Size {
            width: self.width.saturating_sub(value),
            height: self.height.saturating_sub(value),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SizeParseError {
    EmptyString,
    InvalidWidth(u8),
    InvalidHeight(u8),
    HeightMissing,
    MissingSeparator(u8),
    InvalidSize(u8),
}

impl Display for SizeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeParseError::EmptyString => write!(f, "Emptry string (expecting a size in the format 'width x height')"),
            SizeParseError::InvalidWidth(c) => write!(f, "Invalid width (expecting a number formed out of digits but found the character {})", *c as char),
            SizeParseError::InvalidHeight(c) => write!(f, "Invalid height (expecting a number formed out of digits but found the character {})", *c as char),
            SizeParseError::HeightMissing => write!(f, "Incomplete size (height is missing)"),
            SizeParseError::MissingSeparator(c) => write!(f, "Incomplete size (missing separator between width and height - expecting 'x' or ',' but found the character {})", *c as char),
            SizeParseError::InvalidSize(c) => write!(f, "Invalid size (expecting only width and height - but found: {} characters at the end)", *c as char),
        }
    }
}



impl FromStr for Size {
    type Err = SizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = s.as_bytes();
        let mut index = 0;
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }
        if index >= buf.len() {
            return Err(SizeParseError::EmptyString);
        }
        let mut width = 0;
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return Err(SizeParseError::InvalidWidth(buf[index]));
        }
        while index < buf.len() && (buf[index] >= b'0') && (buf[index] <= b'9') {
            width = width * 10 + (buf[index] - b'0') as u32;
            index += 1;
        }
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }        
        if index >= buf.len() {
            return Err(SizeParseError::HeightMissing);
        }
        if (buf[index] != b'x') && (buf[index] != b'X') && (buf[index] != b',') {
            return Err(SizeParseError::MissingSeparator(buf[index]));
        }
        index += 1;
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }        
        if index >= buf.len() {
            return Err(SizeParseError::HeightMissing);
        }
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return Err(SizeParseError::InvalidHeight(buf[index]));
        }
        let mut height = 0;
        while index < buf.len() && (buf[index] >= b'0') && (buf[index] <= b'9') {
            height = height * 10 + (buf[index] - b'0') as u32;
            index += 1;
        }
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }
        if index != buf.len() {
            return Err(SizeParseError::InvalidSize(buf[index]));
        }
        Ok(Size::new(width, height))
    }
}