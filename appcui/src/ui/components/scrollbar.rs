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
    max_value: u64,
    value: u64,
    status: MouseOnScrollbarStatus,
}
impl ScrollBar {
    pub fn new(x: i32, y: i32, dimension: u16, vertical: bool, value: u64, max_value: u64) -> Self {
        Self {
            x,
            y,
            vertical,
            enabled: max_value > 0,
            value: value.min(max_value),
            visible: true,
            max_value,
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
        self.value = value.min(self.max_value);
    }
    #[inline(always)]
    pub fn set_max_value(&mut self, value: u64) {
        self.max_value = value;
        self.value = self.value.min(self.max_value);
    }
    #[inline(always)]
    pub fn update_max_value(&mut self, visible_amount: u64, total_amount: u64) {
        if visible_amount < total_amount {
            self.max_value = total_amount - visible_amount;
        } else {
            self.max_value = 0;
        }
        self.value = self.value.min(self.max_value);
        self.enabled = self.max_value > 0;
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

        let col_maximize_arrow = theme.scrollbar.arrow.normal;
        let col_bar = theme.scrollbar.bar.normal;
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
                    self.y + 1 + self.value_to_offset(),
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
                    self.x + 1 + self.value_to_offset(),
                    self.y,
                    Character::with_attributes(SpecialChar::BlockCentered, col_bar),
                );
            }
        }
    }
    #[inline(always)]
    fn value_to_offset(&self) -> i32 {
        if (self.max_value < 1) || (self.dimension <= 3) {
            return 0;
        }
        let dim = (self.dimension as u64) - 2;
        if self.max_value > 0x0000_0FFF_FFFF_FFFFu64 {
            ((((self.value as u128) * (dim as u128)) / (self.max_value as u128)) as u16) as i32
        } else {
            (((self.value * dim) / self.max_value) as u16) as i32
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
            max_value: 0,
            value: 0,
            status: MouseOnScrollbarStatus::None,
        }
    }
}
