use super::{Size, Point};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Alignament {
    TopLeft = 0,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Center,
}
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left: left.min(right),
            top: top.min(bottom),
            right: right.max(left),
            bottom: bottom.max(top),
        }
    }
    pub fn with_size(x: i32, y: i32, width: u16, height: u16) -> Self {
        Self {
            left: x,
            top: y,
            right: x + (width as i32).max(1) - 1,
            bottom: y + (height as i32).max(1) - 1,
        }
    }
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
    pub fn with_point_and_size(point: Point, size: Size)->Self {
        Rect {
            left: point.x,
            top: point.y,
            right: point.x + (size.width as i32).max(1) - 1,
            bottom: point.y + (size.height as i32).max(1) - 1,
        }
    }
    #[inline]
    pub fn left(&self) -> i32 {
        self.left
    }
    #[inline]
    pub fn right(&self) -> i32 {
        self.right
    }
    #[inline]
    pub fn top(&self) -> i32 {
        self.top
    }
    #[inline]
    pub fn bottom(&self) -> i32 {
        self.bottom
    }
    #[inline]
    pub fn width(&self) -> u32 {
        ((self.right - self.left) as u32) + 1u32
    }
    #[inline]
    pub fn height(&self) -> u32 {
        ((self.bottom - self.top) as u32) + 1u32
    }
    #[inline]
    pub fn center_x(&self) -> i32 {
        (self.right+self.left)/2
    }
    #[inline]
    pub fn center_y(&self) -> i32 {
        (self.bottom+self.top)/2
    }
}
