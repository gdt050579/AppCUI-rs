use crate::graphics::*;
use crate::system::Theme;

#[repr(u8)]
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
            dimension: dimension.min(3),
            status: MouseOnScrollbarStatus::None,
        }
    }
    #[inline(always)]
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
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
    pub fn set_position(&mut self, x: i32, y: i32, dimension: u16) {
        self.x = x;
        self.y = y;
        self.dimension = dimension.min(3);
    }
    pub fn paint(&self, surface: &mut Surface, theme: &Theme) {
        let col_minimize_arrow = theme.scrollbar.arrow.normal;
        let col_maximize_arrow = theme.scrollbar.arrow.normal;
        let col_bar = theme.scrollbar.bar.normal;
        if self.vertical {
            let bottom_y = self.y + (self.dimension as i32) - 1;
            surface.fill_vertical_line(self.x, self.y, bottom_y, Character::with_attributes(SpecialChar::Block50, col_bar));
            surface.write_char(self.x, self.y, Character::with_attributes(SpecialChar::ArrowUp, col_minimize_arrow));
            surface.write_char(self.x, bottom_y, Character::with_attributes(SpecialChar::ArrowDown, col_maximize_arrow));
        } else {
            let right_x = self.x + (self.dimension as i32) - 1;
            surface.fill_horizontal_line(self.x, self.y, right_x, Character::with_attributes(SpecialChar::Block50, col_bar));
            surface.write_char(self.x, self.y, Character::with_attributes(SpecialChar::ArrowLeft, col_minimize_arrow));
            surface.write_char(right_x, self.y, Character::with_attributes(SpecialChar::ArrowRight, col_maximize_arrow));
        }
    }
}
