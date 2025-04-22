use EnumBitFlags::EnumBitFlags;

use super::{Group, GroupPosition, PositionHelper};
use crate::prelude::RuntimeManager;
use crate::system::Handle;
use crate::ui::common::UIElement;

#[EnumBitFlags(bits = 8)]
enum StatusFlags {
    Visible = 0x01,
    OutsideDrawingArea = 0x02,
    SeparatorOnLeft = 0x04,
    SeparatorOnRight = 0x08,
    LeftGroupMarker = 0x10,
    RightGroupMarker = 0x20,
    NoMarker = 0x40,
}
pub(crate) struct ItemBase {
    x: i32,
    y: i32,
    width: u16,
    group: Group,
    status: StatusFlags,
    tooltip: String,
    handle: Handle<UIElement>,
    window: Handle<UIElement>,
}

impl ItemBase {
    // pub(super) fn with_tooltip(part_of_group: bool, tooltip: &str) -> ItemBase {
    //     let mut base = ItemBase::new(part_of_group, true);
    //     base.tooltip.push_str(tooltip);
    //     base
    // }
    pub(super) fn with_width(width: u16, tooltip: &str, visible: bool) -> ItemBase {
        let mut base = ItemBase::new(visible);
        base.width = width;
        base.status |= StatusFlags::NoMarker;
        base.tooltip.push_str(tooltip);
        base
    }
    pub(super) fn new(visible: bool) -> ItemBase {
        ItemBase {
            x: 0,
            y: 0,
            width: 0,
            group: Group::default(),
            tooltip: String::new(),
            handle: Handle::None,
            window: Handle::None,
            status: if visible { StatusFlags::Visible } else { StatusFlags::None },
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
                | StatusFlags::RightGroupMarker
                | StatusFlags::SeparatorOnLeft
                | StatusFlags::SeparatorOnRight,
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
    pub(crate) fn can_be_drawn(&self) -> bool {
        (self.status & (StatusFlags::Visible | StatusFlags::OutsideDrawingArea)) == StatusFlags::Visible
    }
    #[inline(always)]
    pub(crate) fn get_position(&self) -> GroupPosition {
        self.group.pos
    }
    #[inline(always)]
    pub(crate) fn get_group_id(&self) -> u8 {
        self.group.id
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
    pub(crate) fn supports_markers(&self) -> bool {
        !self.status.contains(StatusFlags::NoMarker)
    }
    #[inline(always)]
    pub(crate) fn has_left_separator(&self) -> bool {
        self.status.contains(StatusFlags::SeparatorOnLeft)
    }
    #[inline(always)]
    pub(crate) fn has_right_separator(&self) -> bool {
        self.status.contains(StatusFlags::SeparatorOnRight)
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
    pub(super) fn get_left(&self) -> i32 {
        self.x
    }
    #[inline(always)]
    pub(super) fn get_right(&self) -> i32 {
        self.x + (self.width as i32)
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
    pub(crate) fn set_width(&mut self, value: u16) {
        self.width = value;
    }
    #[inline(always)]
    pub(crate) fn contains(&self, x: i32, y: i32) -> bool {
        (y == self.y)
            && (x >= self.x)
            && (x < (self.x + (self.width as i32)))
            && ((self.status & (StatusFlags::Visible | StatusFlags::OutsideDrawingArea)) == StatusFlags::Visible)
    }
    #[inline(always)]
    pub(crate) fn get_tooltip(&self) -> &str {
        &self.tooltip
    }
    #[inline(always)]
    pub(crate) fn set_tooltip(&mut self, content: &str) {
        self.tooltip.clear();
        self.tooltip.push_str(content);
    }
    #[inline(always)]
    pub(crate) fn get_handle(&self) -> Handle<UIElement> {
        self.handle
    }
    #[inline(always)]
    pub(crate) fn set_handle(&mut self, handle: Handle<UIElement>) {
        self.handle = handle;
    }
    #[inline(always)]
    pub(crate) fn set_window_handle(&mut self, handle: Handle<UIElement>) {
        self.window = handle;
    }
    pub(crate) fn request_recompute_layout(&mut self) {
        let controls = RuntimeManager::get().get_controls_mut();
        if let Some(win) = controls.get_mut(self.window) {
            let size = win.base().size();
            win.control_mut().on_resize(size, size);
        }
    }

    #[inline(always)]
    fn compute_extra_space(&self, helper: &mut PositionHelper) -> i32 {
        // marker size
        let marker_size = if self.status.contains_one(StatusFlags::NoMarker) { 0 } else { 1 };
        let group_space = if helper.last_handle.is_none() {
            0
        } else if self.group.id != helper.last_group {
            if helper.last_group_supports_markers {
                2
            } else {
                1
            }
        } else {
            0
        };
        marker_size + group_space
    }
    pub(super) fn update_position_from_left(&mut self, helper: &mut PositionHelper, right: i32) -> Handle<UIElement> {
        // in case of new group `[=` ==> 2 chars
        // in case of existing group `|` ==> 1 char
        let extra = self.compute_extra_space(helper);
        // I need to check if there is space for: [extra][me][separator or final ']']
        if extra + (self.width as i32) + 2 + helper.x >= right {
            // we can not add this to the view
            self.status |= StatusFlags::OutsideDrawingArea;
            return Handle::None;
        }
        // if all is good, send the last object handle is the group is different
        let previous_handle = if self.group.id != helper.last_group {
            helper.last_handle
        } else {
            Handle::None
        };
        // can be added
        self.x = helper.x + extra;
        self.y = helper.y;
        self.status |= if self.group.id != helper.last_group {
            StatusFlags::LeftGroupMarker
        } else {
            StatusFlags::SeparatorOnLeft
        };
        helper.x += extra + (self.width as i32);
        helper.last_group = self.group.id;
        helper.last_handle = self.handle;
        helper.last_group_supports_markers = self.supports_markers();
        previous_handle
    }
    pub(super) fn update_position_from_right(&mut self, helper: &mut PositionHelper, left: i32) -> Handle<UIElement> {
        // in case of new group `[=` ==> 2 chars
        // in case of existing group `|` ==> 1 char
        let extra = self.compute_extra_space(helper);
        // I need to check if there is space for: [extra][me][separator or final ']']
        if helper.x - (extra + (self.width as i32) + 2) <= left {
            // we can not add this to the view
            self.status |= StatusFlags::OutsideDrawingArea;
            return Handle::None;
        }
        // if all is good, send the last object handle is the group is different
        let previous_handle = if self.group.id != helper.last_group {
            helper.last_handle
        } else {
            Handle::None
        };
        // can be added
        self.x = helper.x - (extra + (self.width as i32));
        self.y = helper.y;
        self.status |= if self.group.id != helper.last_group {
            StatusFlags::RightGroupMarker
        } else {
            StatusFlags::SeparatorOnRight
        };
        helper.x = self.x;
        helper.last_group = self.group.id;
        helper.last_handle = self.handle;
        helper.last_group_supports_markers = self.supports_markers();
        previous_handle
    }
}


macro_rules! add_toolbaritem_basic_methods {
    () => {
        /// Sets the tooltip text for the toolbar item.
        /// 
        /// This method allows you to set a tooltip that will be displayed when the user hovers over the toolbar item.
        /// The tooltip text will be displayed in a small popup window that appears near the item.
        /// 
        /// # Parameters
        /// 
        /// - `text`: A string slice that contains the tooltip text.
        #[inline(always)]
        pub fn set_tooltip(&mut self, text: &str) {
            self.base.set_tooltip(text);
        }

        /// Gets the tooltip text for the toolbar item.
        #[inline(always)]
        pub fn get_tooltip(&self) -> &str {
            self.base.get_tooltip()
        }

        /// Returns **true** if the toolbar item is visible, **false** otherwise.
        #[inline(always)]
        pub fn is_visible(&self) -> bool {
            self.base.is_visible()
        }   

        /// Sets the visibility of the toolbar item.
        /// 
        /// This method allows you to control the visibility of the toolbar item.
        /// When set to **true**, the item will be displayed on the toolbar.        
        #[inline(always)]
        pub fn set_visible(&mut self, visible: bool) {
            self.base.set_visible(visible);
            self.base.request_recompute_layout();
        }  
    };
}