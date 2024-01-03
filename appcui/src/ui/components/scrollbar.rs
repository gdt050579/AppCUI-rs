use crate::graphics::*;
use crate::prelude::ControlBase;
use crate::system::Theme;

#[repr(u8)]
#[derive(Eq, PartialEq)]
enum MouseOnScrollbarStatus {
    None,
    HoverOnMinimizeArrow,
    PressedOnMinimizeArrow,
    HoverOnMaximizeArrow,
    PressedOnMaximizeArrow,
    HoverOnBar,
    PressedOnBar,
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
    pub fn set_value(&mut self, value: u64) {
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
            _ if control_has_focus => theme.scrollbar.arrow.focused,
            _ => theme.scrollbar.arrow.normal,
        };
        let col_maximize_arrow = match () {
            _ if inactive => theme.scrollbar.arrow.inactive,
            _ if self.status == MouseOnScrollbarStatus::HoverOnMaximizeArrow => theme.scrollbar.arrow.hovered,
            _ if control_has_focus => theme.scrollbar.arrow.focused,
            _ => theme.scrollbar.arrow.normal,
        };
        let col_bar = match () {
            _ if inactive => theme.scrollbar.arrow.inactive,
            _ if self.status == MouseOnScrollbarStatus::HoverOnBar => theme.scrollbar.arrow.hovered,
            _ if control_has_focus => theme.scrollbar.arrow.focused,
            _ => theme.scrollbar.arrow.normal,
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
                surface.write_char(
                    self.x,
                    self.y + 1 + self.index_to_screen_offset(),
                    Character::with_attributes(SpecialChar::BlockCentered, col_bar),
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
                surface.write_char(
                    self.x + 1 + self.index_to_screen_offset(),
                    self.y,
                    Character::with_attributes(SpecialChar::BlockCentered, col_bar),
                );
            }
        }
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
