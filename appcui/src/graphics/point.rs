#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
