#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
