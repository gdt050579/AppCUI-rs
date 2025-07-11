use std::ops::{Deref, DerefMut};

use super::generic_scrollbar::*;
use super::ProcessEventResult;
use crate::graphics::*;
use crate::prelude::{ControlBase, MouseEvent};
use crate::system::*;

pub struct HScrollBar {
    base: GenericScrollBar,
}
impl HScrollBar {
    pub fn new(visible: bool) -> Self {
        Self {
            base: GenericScrollBar::new(visible),
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
                self.x + 1 + self.value_to_screen_offset(),
                self.y,
                Character::with_attributes(SpecialChar::BlockCentered, col_position),
            );
        }
    }
    #[inline(always)]
    pub(crate) fn recompute_position(&mut self, pos: i32, available_space: i32, control_size: Size) -> i32 {
        self.base.recompute_layout(pos, control_size.height as i32, available_space as u16) as i32
    }
    #[inline(always)]
    pub(crate) fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        self.base.process_mouse_event(event, false)
    }
}
impl Deref for HScrollBar {
    type Target = GenericScrollBar;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for HScrollBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
