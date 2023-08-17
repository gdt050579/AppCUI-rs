#[repr(u8)]
#[derive(Copy, Clone)]
pub(super) enum Anchors {
    None = 0,

    Left = 0x01,
    Top = 0x02,
    Right = 0x04,
    Bottom = 0x08,

    // 2 anchors
    LeftRight = 0x05, // Left | Right
    TopBottom = 0x0A, // Top  | Bottom,

    // Corners
    TopLeft = 0x03,     // Top | Left
    TopRight = 0x06,    // Top | Right
    BottomLeft = 0x09,  // Bottom | Left
    BottomRight = 0x0C, // Bottom | Right

    // Three
    LeftTopRight = 0x07,    // Left | Top | Right
    LeftBottomRight = 0x0D, // Left | Bottom | Right
    TopLeftBottom = 0x0B,   // Top | Left | Bottom
    TopRightBottom = 0x0E,  // Top | Right | Bottom

    // All
    All = 0x0F,
}
impl Anchors {
    pub (super) fn new(left: bool, top: bool, right: bool, bottom: bool) -> Anchors {
        let mut flags = 0u8;
        flags |= if left { Anchors::Left as u8 } else { 0 };
        flags |= if right { Anchors::Right as u8 } else { 0 };
        flags |= if top { Anchors::Top as u8 } else { 0 };
        flags |= if bottom { Anchors::Bottom as u8 } else { 0 };
        match flags {
            0 => Anchors::None,

            0x01 => Anchors::Left,
            0x02 => Anchors::Top,
            0x03 => Anchors::TopLeft,
            0x04 => Anchors::Right,
            0x05 => Anchors::LeftRight,
            0x06 => Anchors::TopRight,
            0x07 => Anchors::LeftTopRight,
            0x08 => Anchors::Bottom,
            0x09 => Anchors::BottomLeft,
            0x0A => Anchors::TopBottom,
            0x0B => Anchors::TopLeftBottom,
            0x0C => Anchors::BottomRight,
            0x0D => Anchors::LeftBottomRight,
            0x0E => Anchors::TopRightBottom,
            0x0F => Anchors::All,

            _ => Anchors::None,
        }
    }
}
