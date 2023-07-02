use EnumBitFlags::EnumBitFlags;

use crate::{
    graphics::{CharAttribute, Character, SpecialChar, Surface, TextAlignament, TextFormat},
    system::Theme,
    utils::Caption,
};

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
pub(super) struct Position {
    x: i32,
    y: i32,
    width: u16,
    layout: ToolbarItemLayout,
    status: StatusFlags,    
}

impl Position {
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
    pub(super) fn set_right_marker(&mut self) {
        self.status |= StatusFlags::RightGroupMarker;
    }
    #[inline(always)]
    pub(super) fn set_left_marker(&mut self) {
        self.status |= StatusFlags::LeftGroupMarker;
    }
    pub(super) fn update_position_from_left(
        &mut self,
        x: i32,
        y: i32,
        last: DecoratorType,
    ) -> (i32, bool) {
        let part_of_group = self.status.contains(StatusFlags::ParOfGroup);
        let mut extra_space = 0;
        let mut right_group_marker = false;
        if part_of_group {
            extra_space = 1;
            if self.decorator_type != last {
                right_group_marker = true;
                extra_space += 1;
            }
        } else {
            if last != DecoratorType::None {
                right_group_marker = true;
            }
        }
        self.y = y;
        self.x = x + extra_space;
        let next = self.x + (self.width as i32);
        if part_of_group && (self.decorator_type != last) {
            self.status |= StatusFlags::LeftGroupMarker;
        }

        (next, right_group_marker)
    }
    pub(super) fn update_position_from_right(
        &mut self,
        x: i32,
        y: i32,
        last: DecoratorType,
    ) -> (i32, bool) {
        let part_of_group = self.status.contains(StatusFlags::ParOfGroup);
        let mut extra_space = 0;
        let mut left_group_marker = false;
        if part_of_group {
            extra_space = 1;
            if self.decorator_type != last {
                left_group_marker = true;
                extra_space += 1;
            }
        } else {
            if last != DecoratorType::None {
                left_group_marker = true;
            }
        }
        self.y = y;
        self.x = (x - self.width as i32) + 1;
        self.x -= extra_space;
        let next = self.x - 1;
        if part_of_group && (self.decorator_type != last) {
            self.status |= StatusFlags::RightGroupMarker;
        }

        (next, left_group_marker)
    }
}