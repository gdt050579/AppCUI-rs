use EnumBitFlags::EnumBitFlags;

use crate::{
    graphics::{CharAttribute, Character, SpecialChar, Surface, TextAlignament, TextFormat},
    system::Theme,
    utils::Caption,
};

use super::ToolBarItem;

// #[repr(u8)]
// #[derive(Clone, Copy, PartialEq)]
// pub(super) enum DecoratorType {
//     None,
//     HotKeY,
//     CloseButton,
//     MaximizeRestoreButton,
//     WindowResize,
//     Tag,
//     Button,
//     SingleChoice,
//     CheckBox,
//     Text,
// }

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
    ParOfGroup = 0x04,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(super) struct ItemBase {
    x: i32,
    y: i32,
    width: u16,
    layout: ToolbarItemLayout,
    status: StatusFlags,
}

impl ItemBase {
    pub(super) fn new(layout: ToolbarItemLayout, part_of_group: bool) -> ItemBase {
        ItemBase {
            x: 0,
            y: 0,
            width: 0,
            layout: layout,
            status: if part_of_group {
                StatusFlags::ParOfGroup | StatusFlags::Visible
            } else {
                StatusFlags::Visible
            },
        }
    }

    #[inline(always)]
    pub(super) fn clear(&mut self) {
        self.status.remove(
            StatusFlags::Visible | StatusFlags::LeftGroupMarker | StatusFlags::RightGroupMarker,
        );
    }
    #[inline(always)]
    pub(super) fn set_visible(&mut self) {
        self.status |= StatusFlags::Visible;
    }
    #[inline(always)]
    pub(super) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub(super) fn is_hidden(&self) -> bool {
        self.status.contains(StatusFlags::Hidden)
    }
    #[inline(always)]
    pub(super) fn get_layout(&self) -> ToolbarItemLayout {
        self.layout
    }
    #[inline(always)]
    pub(super) fn has_right_group_marker(&self) -> bool {
        self.status.contains(StatusFlags::RightGroupMarker)
    }
    #[inline(always)]
    pub(super) fn has_left_group_marker(&self) -> bool {
        self.status.contains(StatusFlags::LeftGroupMarker)
    }
    #[inline(always)]
    pub(super) fn set_right_marker(&mut self) {
        self.status |= StatusFlags::RightGroupMarker;
    }
    #[inline(always)]
    pub(super) fn set_left_marker(&mut self) {
        self.status |= StatusFlags::LeftGroupMarker;
    }
    #[inline(always)]
    pub(super) fn get_x(&self) -> i32 {
        self.x
    }
    #[inline(always)]
    pub(super) fn get_y(&self) -> i32 {
        self.y
    }
    #[inline(always)]
    pub(super) fn get_width(&self) -> i32 {
        self.width as i32
    }
    #[inline(always)]
    pub(super) fn set_width(&mut self, value: u16) {
        self.width = value;
    }
    #[inline(always)]
    pub(super) fn is_part_of_group(&self) -> bool {
        self.status.contains(StatusFlags::ParOfGroup)
    }
    pub(super) fn update_position_from_left(
        &mut self,
        x: i32,
        y: i32,
        my_variant: Option<std::mem::Discriminant<ToolBarItem>>,
        last: Option<std::mem::Discriminant<ToolBarItem>>,
    ) -> (i32, bool) {
        let part_of_group = self.status.contains(StatusFlags::ParOfGroup);
        let mut extra_space = 0;
        let mut right_group_marker = false;

        if part_of_group {
            extra_space = 1;
            if my_variant != last {
                right_group_marker = true;
                extra_space += 1;
            }
        } else {
            right_group_marker = last.is_some();
        }
        self.y = y;
        self.x = x + extra_space;
        let next = self.x + (self.width as i32);
        if part_of_group && (my_variant != last) {
            self.status |= StatusFlags::LeftGroupMarker;
        }

        (next, right_group_marker)
    }
    pub(super) fn update_position_from_right(
        &mut self,
        x: i32,
        y: i32,
        my_variant: Option<std::mem::Discriminant<ToolBarItem>>,
        last: Option<std::mem::Discriminant<ToolBarItem>>,
    ) -> (i32, bool) {
        let part_of_group = self.status.contains(StatusFlags::ParOfGroup);
        let mut extra_space = 0;
        let mut left_group_marker = false;
        if part_of_group {
            extra_space = 1;
            if my_variant != last {
                left_group_marker = true;
                extra_space += 1;
            }
        } else {
            left_group_marker = last.is_some();
        }
        self.y = y;
        self.x = (x - self.width as i32) + 1;
        self.x -= extra_space;
        let next = self.x - 1;
        if part_of_group && (my_variant != last) {
            self.status |= StatusFlags::RightGroupMarker;
        }

        (next, left_group_marker)
    }
}
