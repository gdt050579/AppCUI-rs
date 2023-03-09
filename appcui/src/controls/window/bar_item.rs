use EnumBitFlags::EnumBitFlags;

use crate::{utils::Caption, graphics::Surface, system::Theme};

#[repr(u8)]
pub(super) enum BarItemType {
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
enum BarItemLayout {
    None,
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    Hidden = 0x02,
    Checked = 0x04,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(super) struct BarItem {
    tooltip: String,
    text: Caption,
    item_type: BarItemType,
    status: StatusFlags,
    x: i32,
    y: i32,
    width: u16,
    id: u32,
    layout: BarItemLayout,
}

impl BarItem {
    #[inline(always)]
    pub (super) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub (super) fn is_hidden(&self) -> bool {
        self.status.contains(StatusFlags::Hidden)
    }
    #[inline(always)]
    pub (super) fn is_checked(&self) -> bool {
        self.status.contains(StatusFlags::Checked)
    }
    #[inline(always)]
    pub (super) fn center_x(&self) -> i32 {
        self.x + ((self.width / 2) as i32)
    }
    #[inline(always)]
    pub (super) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::Hidden))
                == StatusFlags::Visible)
    }
    pub (super) fn paint(&self, surface: &mut Surface, theme: &Theme) {

    }
}
// inline void SetFlag(WindowBarItemFlags flg)
// {
//     Flags = static_cast<WindowBarItemFlags>(((unsigned char) Flags) | ((unsigned char) flg));
// }
// inline void RemoveFlag(WindowBarItemFlags flg)
// {
//     Flags = static_cast<WindowBarItemFlags>(((unsigned char) Flags) & (~((unsigned char) flg)));
// }
