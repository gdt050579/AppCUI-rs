use EnumBitFlags::EnumBitFlags;

use super::{GroupPosition, ToolBarItem, Group};

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    OutsideDrawingArea = 0x02,
    ParOfGroup = 0x04,
    LeftGroupMarker = 0x08,
    RightGroupMarker = 0x10,
}
pub(crate) struct ItemBase {
    x: i32,
    y: i32,
    width: u16,
    group: Group,
    status: StatusFlags,
    tooltip: String,
}

impl ItemBase {
    pub(super) fn with_tooltip(part_of_group: bool, tooltip: &str) -> ItemBase {
        let mut base = ItemBase::new(part_of_group, true);
        base.tooltip.push_str(tooltip);
        base
    }
    pub(super) fn with_width(
        width: u16,
        tooltip: &str,
        visible: bool,
    ) -> ItemBase {
        let mut base = ItemBase::new(false, visible);
        base.width = width;
        base.tooltip.push_str(tooltip);
        base
    }
    pub(super) fn new(part_of_group: bool, visible: bool) -> ItemBase {
        ItemBase {
            x: 0,
            y: 0,
            width: 0,
            group: Group::default(),
            tooltip: String::new(),
            status: if part_of_group {
                if visible {
                    StatusFlags::ParOfGroup | StatusFlags::Visible
                } else {
                    StatusFlags::ParOfGroup
                }
            } else {
                if visible {
                    StatusFlags::Visible
                } else {
                    StatusFlags::None
                }
            },
        }
    }
    #[inline(always)]
    pub(crate) fn update_group(&mut self, group: Group) {
        self.group = group;
    }
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        self.status.remove(
            StatusFlags::OutsideDrawingArea
                | StatusFlags::LeftGroupMarker
                | StatusFlags::RightGroupMarker,
        );
    }
    #[inline(always)]
    pub(crate) fn set_visible(&mut self, value: bool) {
        if value {
            self.status |= StatusFlags::Visible;
        } else {
            self.status.remove(StatusFlags::Visible);
        }
    }
    #[inline(always)]
    pub(crate) fn is_visible(&self) -> bool {
        self.status.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub(crate) fn set_outside_drawing_area(&mut self) {
        self.status |= StatusFlags::OutsideDrawingArea;
    }
    #[inline(always)]
    pub(crate) fn can_be_drawn(&self) -> bool {
        (self.status & (StatusFlags::Visible | StatusFlags::OutsideDrawingArea))
            == StatusFlags::Visible
    }
    #[inline(always)]
    pub(crate) fn get_gravity(&self) -> GroupPosition {
        self.group.pos
    }
    #[inline(always)]
    pub(crate) fn has_right_group_marker(&self) -> bool {
        self.status.contains(StatusFlags::RightGroupMarker)
    }
    #[inline(always)]
    pub(crate) fn has_left_group_marker(&self) -> bool {
        self.status.contains(StatusFlags::LeftGroupMarker)
    }
    #[inline(always)]
    pub(crate) fn set_right_marker(&mut self) {
        self.status |= StatusFlags::RightGroupMarker;
    }
    #[inline(always)]
    pub(crate) fn set_left_marker(&mut self) {
        self.status |= StatusFlags::LeftGroupMarker;
    }
    #[inline(always)]
    pub(super) fn get_x(&self) -> i32 {
        self.x
    }
    #[inline(always)]
    pub(crate) fn get_y(&self) -> i32 {
        self.y
    }
    #[inline(always)]
    pub(crate) fn center_x(&self) -> i32 {
        self.x + ((self.width / 2) as i32)
    }
    #[inline(always)]
    pub(crate) fn get_width(&self) -> i32 {
        self.width as i32
    }
    #[inline(always)]
    pub(crate) fn set_width(&mut self, value: u16) {
        self.width = value;
    }
    #[inline(always)]
    pub(crate) fn is_part_of_group(&self) -> bool {
        self.status.contains(StatusFlags::ParOfGroup)
    }
    #[inline(always)]
    pub(crate) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::OutsideDrawingArea))
                == StatusFlags::Visible)
    }
    #[inline(always)]
    pub(crate) fn get_tooltip(&self) -> &str {
        &self.tooltip
    }

    pub(crate) fn request_recompute_layout(&mut self) {

    }

    pub(super) fn update_position_from_left(
        &mut self,
        x: i32,
        y: i32,
        my_variant: Option<std::mem::Discriminant<ToolBarItem>>,
        last: Option<std::mem::Discriminant<ToolBarItem>>,
    ) -> (i32, bool, bool) {
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
        (next, right_group_marker, !part_of_group)
    }
    pub(super) fn update_position_from_right(
        &mut self,
        x: i32,
        y: i32,
        my_variant: Option<std::mem::Discriminant<ToolBarItem>>,
        last: Option<std::mem::Discriminant<ToolBarItem>>,
    ) -> (i32, bool, bool) {
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

        (next, left_group_marker, !part_of_group)
    }
}
