
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
