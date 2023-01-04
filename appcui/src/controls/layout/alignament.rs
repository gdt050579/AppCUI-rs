#[repr(u8)]
pub(super) enum Alignament {
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
