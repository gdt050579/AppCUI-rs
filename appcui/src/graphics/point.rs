/// This module defines a `Point` struct representing a point in 2D space with x and y coordinates.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point {
    /// Horizontal coordinate in character cells.
    pub x: i32,
    /// Vertical coordinate in character cells.
    pub y: i32,
}
impl Point {
    /// The origin point (0, 0).
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    
    /// Creates a new `Point` with the specified x and y coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
