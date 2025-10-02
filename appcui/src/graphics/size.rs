use std::str::FromStr;


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

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = s.as_bytes();
        let mut index = 0;
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }
        if index >= buf.len() {
            return Err("Emptry string (expecting a size in the format 'width x height')".to_string());
        }
        let mut width = 0;
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return Err(format!("Invalid width (expecting a number formed out of digits but found the character {})", buf[index] as char));
        }
        while index < buf.len() && (buf[index] >= b'0') && (buf[index] <= b'9') {
            width = width * 10 + (buf[index] - b'0') as u32;
            index += 1;
        }
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }        
        if index >= buf.len() {
            return Err("Incomplete size (width is present but height is missing)".to_string());
        }
        if (buf[index] != b'x') && (buf[index] != b'X') && (buf[index] != b',') {
            return Err(format!("Invalid size (expecting 'x' or ',' separated width and height but found the character {})", buf[index] as char));
        }
        index += 1;
        while index < buf.len() && ((buf[index] == b' ') || (buf[index] == b'\t')) {
            index += 1;
        }        
        if index >= buf.len() {
            return Err("Incomplete size (height is missing)".to_string());
        }
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return Err(format!("Invalid height (expecting a number formed out of digits but found the character {})", buf[index] as char));
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
            return Err(format!("Invalid size (expecting only width and height - but found: {} at the end)", &s[index..]));
        }
        Ok(Size::new(width, height))
    }
}