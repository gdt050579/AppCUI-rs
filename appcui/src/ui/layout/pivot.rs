#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pivot {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
    BottomCenter = 4,
    CenterLeft = 5,
    CenterRight = 6,
    TopCenter = 7,
    Center = 8,
}