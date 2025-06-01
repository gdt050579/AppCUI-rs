use super::{Size, Point};

/// Alignament enum represents the alignment of a rectangle in a 2D space.
/// It is used to specify how a rectangle or other rectangular objects should be positioned relative to a given point.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Alignament {
    /// The top-left corner of the rectangle.
    /// This is the default alignment.
    TopLeft = 0,
    
    /// The top-center of the rectangle.
    /// This alignment is used to center the rectangle horizontally at the top.
    Top,
    
    /// The top-right corner of the rectangle.
    /// This alignment is used to position the rectangle at the top-right corner.
    TopRight,
    
    /// The right-center of the rectangle.
    /// This alignment is used to center the rectangle vertically on the right side.
    Right,
    
    /// The bottom-right corner of the rectangle.
    /// This alignment is used to position the rectangle at the bottom-right corner.
    BottomRight,
    
    /// The bottom-center of the rectangle.
    /// This alignment is used to center the rectangle horizontally at the bottom.
    Bottom,
    
    /// The bottom-left corner of the rectangle.
    /// This alignment is used to position the rectangle at the bottom-left corner.
    BottomLeft,

    /// The left-center of the rectangle.
    /// This alignment is used to center the rectangle vertically on the left side.
    Left,

    /// The center of the rectangle.
    /// This alignment is used to center the rectangle both horizontally and vertically.
    Center,
}

/// A rectangle defined by its left, top, right, and bottom coordinates.
/// The coordinates are automatically adjusted to ensure that left <= right and top <= bottom.
#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
impl Rect {
    /// Creates a new rectangle with the specified coordinates. The coordinates are automatically
    /// adjusted to ensure that left <= right and top <= bottom.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left: left.min(right),
            top: top.min(bottom),
            right: right.max(left),
            bottom: bottom.max(top),
        }
    }

    /// Creates a new rectangle with the specified coordinates and size. 
    pub fn with_size(x: i32, y: i32, width: u16, height: u16) -> Self {
        Self {
            left: x,
            top: y,
            right: x + (width as i32).max(1) - 1,
            bottom: y + (height as i32).max(1) - 1,
        }
    }

    /// Creates a new rectangle with the specified coordinates, size and alignament.
    /// The alignament can have the following values:
    /// - `TopLeft`: the x,y coordinates are the top-left corner of the rectangle
    /// - `Top`: the x,y coordinates are the top-center of the rectangle
    /// - `TopRight`: the x,y coordinates are the top-right corner of the rectangle
    /// - `Right`: the x,y coordinates are the right-center of the rectangle
    /// - `BottomRight`: the x,y coordinates are the bottom-right corner of the rectangle
    /// - `Bottom`: the x,y coordinates are the bottom-center of the rectangle
    /// - `BottomLeft`: the x,y coordinates are the bottom-left corner of the rectangle
    /// - `Left`: the x,y coordinates are the left-center of the rectangle
    /// - `Center`: the x,y coordinates are the center of the rectangle
    pub fn with_alignament(x: i32, y: i32, width: u16, height: u16, align: Alignament) -> Self {
        let x = match align {
            Alignament::TopLeft => x,
            Alignament::Top => x - ((width as i32) / 2),
            Alignament::TopRight => x - (width as i32) + 1,
            Alignament::Right => x - (width as i32) + 1,
            Alignament::BottomRight => x - (width as i32) + 1,
            Alignament::Bottom => x - ((width as i32) / 2),
            Alignament::BottomLeft => x,
            Alignament::Left => x,
            Alignament::Center => x - ((width as i32) / 2),
        };
        let y = match align {
            Alignament::TopLeft => y,
            Alignament::Top => y,
            Alignament::TopRight => y,
            Alignament::Right => y - ((height as i32) / 2),
            Alignament::BottomRight => y - (height as i32) + 1,
            Alignament::Bottom => y - (height as i32) + 1,
            Alignament::BottomLeft => y - (height as i32) + 1,
            Alignament::Left => y - ((height as i32) / 2),
            Alignament::Center => y - ((height as i32) / 2),
        };
        Self {
            left: x,
            top: y,
            right: x + (width as i32).max(1) - 1,
            bottom: y + (height as i32).max(1) - 1,
        }
    }

    /// Creates a new rectangle with the specified coordinates and size.
    /// The coordonates are represented by a Point and are considered the top-left corner of the rectangle.
    pub fn with_point_and_size(point: Point, size: Size)->Self {
        Rect {
            left: point.x,
            top: point.y,
            right: point.x + (size.width as i32).max(1) - 1,
            bottom: point.y + (size.height as i32).max(1) - 1,
        }
    }

    /// Returns the rectangle left coordinate.
    #[inline]
    pub fn left(&self) -> i32 {
        self.left
    }

    /// Returns the rectangle right coordinate.
    #[inline]
    pub fn right(&self) -> i32 {
        self.right
    }

    /// Returns the rectangle top coordinate.
    #[inline]
    pub fn top(&self) -> i32 {
        self.top
    }

    /// Returns the rectangle bottom coordinate.
    #[inline]
    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    /// Returns the rectangle width.
    #[inline]
    pub fn width(&self) -> u32 {
        ((self.right - self.left) as u32) + 1u32
    }

    /// Returns the rectangle height.
    #[inline]
    pub fn height(&self) -> u32 {
        ((self.bottom - self.top) as u32) + 1u32
    }

    /// Returns the rectangle X-axis center.
    #[inline]
    pub fn center_x(&self) -> i32 {
        (self.right+self.left)/2
    }

    /// Returns the rectangle Y-axis center.
    #[inline]
    pub fn center_y(&self) -> i32 {
        (self.bottom+self.top)/2
    }
}
