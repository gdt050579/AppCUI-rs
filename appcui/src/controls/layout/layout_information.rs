use super::Alignament;
use super::Coordonate;
use super::Size;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits:16)]
pub enum LayoutInformationFields {
    X = 0x0001,
    Y = 0x0002,
    LEFT = 0x0004,
    RIGHT = 0x0008,
    TOP = 0x0010,
    BOTTOM = 0x0020,
    WIDTH = 0x0040,
    HEIGHT = 0x0080,
    ALIGN = 0x0100,
    DOCK = 0x0200,
}
pub(super) struct LayoutInformation {
    pub x: Coordonate,
    pub y: Coordonate,
    pub width: Size,
    pub height: Size,
    pub a_left: Coordonate,
    pub a_right: Coordonate,
    pub a_top: Coordonate,
    pub a_bottom: Coordonate,
    pub fields: LayoutInformationFields,
    pub align: Alignament,
    pub dock: Alignament,
}

impl LayoutInformation {
    pub (super) fn new() -> LayoutInformation {
        LayoutInformation {
            x: Coordonate::Absolute(0),
            y: Coordonate::Absolute(0),
            width: Size::Absolute(0),
            height: Size::Absolute(0),
            a_left: Coordonate::Absolute(0),
            a_right: Coordonate::Absolute(0),
            a_top: Coordonate::Absolute(0),
            a_bottom: Coordonate::Absolute(0),
            fields: LayoutInformationFields::None,
            align: Alignament::TopLeft,
            dock: Alignament::TopLeft,
        }
    }
}
