use crate::graphics::*;
use crate::prelude::{ControlBase, MouseEvent};
use crate::system::Theme;

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]
enum MouseOnScrollbarStatus {
    None,
    HoverOnMinimizeArrow,
    PressedOnMinimizeArrow,
    HoverOnMaximizeArrow,
    PressedOnMaximizeArrow,
    HoverOnBar,
    PressedOnBar,
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum MousePosition {
    MinimizeArrow,
    MaximizeArrow,
    Bar,
    OutsideScrollBar,
}

pub struct ScrollBar {
    x: i32,
    y: i32,
    dimension: u16,
    vertical: bool,
    enabled: bool,
    visible: bool,
    count: u64,
    index: u64,
    status: MouseOnScrollbarStatus,
}
impl ScrollBar {
    pub fn new(x: i32, y: i32, dimension: u16, vertical: bool, count: u64) -> Self {
        Self {
            x,
            y,
            vertical,
            enabled: count > 0,
            index: 0,
            visible: true,
            count,
            dimension: dimension.max(3),
            status: MouseOnScrollbarStatus::None,
        }
    }
    #[inline(always)]
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    #[inline(always)]
    pub fn set_enabled(&mut self, visible: bool) {
        self.enabled = visible;
    }
    #[inline(always)]
    pub fn set_index(&mut self, value: u64) {
        self.index = if self.count > 0 { value.min(self.count - 1) } else { 0 };
    }
    #[inline(always)]
    pub fn set_count(&mut self, count: u64) {
        self.count = count;
        self.index = if self.count > 0 { self.index.min(self.count - 1) } else { 0 };
        self.enabled = self.count > 0;
    }
    #[inline(always)]
    pub fn update_count(&mut self, visible_indexes: u64, total_indexes: u64) {
        if (visible_indexes < total_indexes) || (visible_indexes == 0) {
            self.count = total_indexes - visible_indexes;
        } else {
            self.count = 0;
        }
        self.index = if self.count > 0 { self.index.min(self.count - 1) } else { 0 };
        self.enabled = self.count > 0;
    }
    #[inline(always)]
    pub fn set_position(&mut self, x: i32, y: i32, dimension: u16) {
        self.x = x;
        self.y = y;
        self.dimension = dimension.max(3);
        if self.dimension <= 3 {
            self.visible = false;
        }
    }
    pub fn update_position(&mut self, control_size: Size, decrease_margin: i32, increase_margin: i32, outside_rectangle: bool) {
        let w = control_size.width as i32;
        let h = control_size.height as i32;
        if self.vertical {
            let dimension = h - (increase_margin + decrease_margin);
            self.x = if outside_rectangle { w } else { w - 1 };
            self.y = decrease_margin;
            self.dimension = dimension.max(3) as u16;
            self.visible = dimension >= 3;
        } else {
            let dimension = w - (increase_margin + decrease_margin);
            self.y = if outside_rectangle { h } else { h - 1 };
            self.x = decrease_margin;
            self.dimension = dimension.max(3) as u16;
            self.visible = dimension >= 3;
        }
        if (w < 1) || (h < 1) {
            self.visible = false;
        }
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme, control: &ControlBase) {
        if !self.visible {
            return;
        }
        let control_has_focus = control.has_focus();
        let inactive = !(control.is_enabled() && self.enabled);
        let col_minimize_arrow = match () {
            _ if inactive => theme.scrollbar.arrow.inactive,
            _ if self.status == MouseOnScrollbarStatus::HoverOnMinimizeArrow => theme.scrollbar.arrow.hovered,
            _ if self.status == MouseOnScrollbarStatus::PressedOnMinimizeArrow => theme.scrollbar.arrow.pressed_or_selectd,
            _ if control_has_focus => theme.scrollbar.arrow.focused,
            _ => theme.scrollbar.arrow.normal,
        };
        let col_maximize_arrow = match () {
            _ if inactive => theme.scrollbar.arrow.inactive,
            _ if self.status == MouseOnScrollbarStatus::HoverOnMaximizeArrow => theme.scrollbar.arrow.hovered,
            _ if self.status == MouseOnScrollbarStatus::PressedOnMaximizeArrow => theme.scrollbar.arrow.pressed_or_selectd,
            _ if control_has_focus => theme.scrollbar.arrow.focused,
            _ => theme.scrollbar.arrow.normal,
        };
        let col_bar = match () {
            _ if inactive => theme.scrollbar.bar.inactive,
            _ if self.status == MouseOnScrollbarStatus::HoverOnBar => theme.scrollbar.bar.hovered,
            _ if self.status == MouseOnScrollbarStatus::PressedOnBar => theme.scrollbar.bar.pressed_or_selectd,
            _ if control_has_focus => theme.scrollbar.bar.focused,
            _ => theme.scrollbar.bar.normal,
        };
        if self.vertical {
            let bottom_y = self.y + (self.dimension as i32) - 1;
            surface.fill_vertical_line(self.x, self.y, bottom_y, Character::with_attributes(SpecialChar::Block50, col_bar));
            surface.write_char(self.x, self.y, Character::with_attributes(SpecialChar::TriangleUp, col_minimize_arrow));
            surface.write_char(
                self.x,
                bottom_y,
                Character::with_attributes(SpecialChar::TriangleDown, col_maximize_arrow),
            );
            if !inactive {
                let col_position = match () {
                    _ if inactive => theme.scrollbar.position.inactive,
                    _ if self.status == MouseOnScrollbarStatus::HoverOnBar => theme.scrollbar.position.hovered,
                    _ if self.status == MouseOnScrollbarStatus::PressedOnBar => theme.scrollbar.position.pressed_or_selectd,
                    _ if control_has_focus => theme.scrollbar.position.focused,
                    _ => theme.scrollbar.position.normal,
                };
                surface.write_char(
                    self.x,
                    self.y + 1 + self.index_to_screen_offset(),
                    Character::with_attributes(SpecialChar::BlockCentered, col_position),
                );
            }
        } else {
            let right_x = self.x + (self.dimension as i32) - 1;
            surface.fill_horizontal_line(self.x, self.y, right_x, Character::with_attributes(SpecialChar::Block50, col_bar));
            surface.write_char(self.x, self.y, Character::with_attributes(SpecialChar::TriangleLeft, col_minimize_arrow));
            surface.write_char(
                right_x,
                self.y,
                Character::with_attributes(SpecialChar::TriangleRight, col_maximize_arrow),
            );
            if !inactive {
                let col_position = match () {
                    _ if inactive => theme.scrollbar.position.inactive,
                    _ if self.status == MouseOnScrollbarStatus::HoverOnBar => theme.scrollbar.position.hovered,
                    _ if self.status == MouseOnScrollbarStatus::PressedOnBar => theme.scrollbar.position.pressed_or_selectd,
                    _ if control_has_focus => theme.scrollbar.position.focused,
                    _ => theme.scrollbar.position.normal,
                };
                surface.write_char(
                    self.x + 1 + self.index_to_screen_offset(),
                    self.y,
                    Character::with_attributes(SpecialChar::BlockCentered, col_position),
                );
            }
        }
    }
    fn mouse_coords_to_scroll_pos(&self, x: i32, y: i32) -> MousePosition {
        if self.vertical {
            match () {
                _ if self.x != x => MousePosition::OutsideScrollBar,
                _ if self.y == y => MousePosition::MinimizeArrow,
                _ if (self.y + (self.dimension as i32) - 1) == y => MousePosition::MaximizeArrow,
                _ if (y > self.y) && (y < (self.y + (self.dimension as i32) - 1)) => MousePosition::Bar,
                _ => MousePosition::OutsideScrollBar,
            }
        } else {
            match () {
                _ if self.y != y => MousePosition::OutsideScrollBar,
                _ if self.x == x => MousePosition::MinimizeArrow,
                _ if (self.x + (self.dimension as i32) - 1) == x => MousePosition::MaximizeArrow,
                _ if (x > self.x) && (x < (self.x + (self.dimension as i32) - 1)) => MousePosition::Bar,
                _ => MousePosition::OutsideScrollBar,
            }
        }
    }
    fn update_hover_status(&mut self, x: i32, y: i32) {
        match self.mouse_coords_to_scroll_pos(x, y) {
            MousePosition::MinimizeArrow => self.status = MouseOnScrollbarStatus::HoverOnMinimizeArrow,
            MousePosition::MaximizeArrow => self.status = MouseOnScrollbarStatus::HoverOnMaximizeArrow,
            MousePosition::Bar => self.status = MouseOnScrollbarStatus::HoverOnBar,
            MousePosition::OutsideScrollBar => self.status = MouseOnScrollbarStatus::None,
        }
    }
    fn get_press_status(&self, x: i32, y: i32) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos(x, y) {
            MousePosition::MinimizeArrow => MouseOnScrollbarStatus::PressedOnMinimizeArrow,
            MousePosition::MaximizeArrow => MouseOnScrollbarStatus::PressedOnMaximizeArrow,
            MousePosition::Bar => MouseOnScrollbarStatus::PressedOnBar,
            MousePosition::OutsideScrollBar => MouseOnScrollbarStatus::None,
        }
    }
    fn update_index_for_mouse_pos(&mut self, x: i32, y: i32, new_status: MouseOnScrollbarStatus) {
        match new_status {
            MouseOnScrollbarStatus::PressedOnMinimizeArrow => {
                if self.index > 0 {
                    self.set_index(self.index - 1);
                }
            }
            MouseOnScrollbarStatus::PressedOnMaximizeArrow => {
                self.set_index(self.index + 1);
            }
            MouseOnScrollbarStatus::PressedOnBar => {
                // compute the new index
            }
            _ => {}
        }
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent) -> bool {
        if !(self.visible && self.enabled) {
            return false;
        }
        match event {
            MouseEvent::Over(data) => {
                self.update_hover_status(data.x, data.y);
            }
            MouseEvent::Pressed(data) => {
                self.status = self.get_press_status(data.x, data.y);
                if self.status != MouseOnScrollbarStatus::None {
                    self.update_index_for_mouse_pos(data.x, data.y, self.status);
                }
            }
            MouseEvent::Released(data) => {
                self.update_hover_status(data.x, data.y);
            }
            MouseEvent::Drag(data) => {
                let new_status = self.get_press_status(data.x, data.y);
                if (new_status == self.status) && (new_status == MouseOnScrollbarStatus::PressedOnBar) {
                    self.update_index_for_mouse_pos(data.x, data.y, MouseOnScrollbarStatus::PressedOnBar);
                }
            }
            _ => {}
        }
        return self.status != MouseOnScrollbarStatus::None;
    }
    #[inline(always)]
    fn index_to_screen_offset(&self) -> i32 {
        if (self.count < 2) || (self.dimension <= 3) {
            return 0;
        }
        let dim = (self.dimension as u64) - 3;
        let cnt = self.count - 1;
        let idx = self.index.min(cnt); // safety check
        if self.count > 0x0000_0FFF_FFFF_FFFFu64 {
            ((((idx as u128) * (dim as u128)) / (cnt as u128)) as u16) as i32
        } else {
            (((idx * dim) / cnt) as u16) as i32
        }
    }
}
impl Default for ScrollBar {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            dimension: 0,
            vertical: false,
            enabled: false,
            visible: false,
            count: 0,
            index: 0,
            status: MouseOnScrollbarStatus::None,
        }
    }
}
