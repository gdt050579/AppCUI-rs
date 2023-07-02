use EnumBitFlags::EnumBitFlags;

use crate::{
    graphics::{CharAttribute, Character, SpecialChar, Surface, TextAlignament, TextFormat},
    system::Theme,
    utils::Caption,
};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub(super) enum DecoratorType {
    None,
    HotKeY,
    CloseButton,
    MaximizeRestoreButton,
    WindowResize,
    Tag,
    Button,
    SingleChoice,
    CheckBox,
    Text,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub(super) enum ToolbarItemLayout {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    Hidden = 0x02,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(super) struct Position {
    x: i32,
    y: i32,
    width: u16,
    layout: ToolbarItemLayout,
    status: StatusFlags,    
}