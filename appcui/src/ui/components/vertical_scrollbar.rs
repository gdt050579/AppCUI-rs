use crate::graphics::*;
use crate::prelude::{ControlBase, MouseEvent};
use crate::system::*;

use super::ProcessEventResult;

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

impl MouseOnScrollbarStatus {
    #[inline(always)]
    fn is_none(&self) -> bool {
        *self == MouseOnScrollbarStatus::None
    }
    #[inline(always)]
    fn is_pressed(&self) -> bool {
        matches!(
            self,
            MouseOnScrollbarStatus::PressedOnMinimizeArrow | MouseOnScrollbarStatus::PressedOnMaximizeArrow | MouseOnScrollbarStatus::PressedOnBar
        )
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum MousePosition {
    MinimizeArrow,
    MaximizeArrow,
    Bar,
    OutsideScrollBar,
}

pub struct VScrollBar {
    x: i32,
    y: i32,
    dimension: u16,
    enabled: bool,
    visible: bool,
    count: u64,
    index: u64,
    status: MouseOnScrollbarStatus,
}
impl VScrollBar {
    pub fn new(visible: bool) -> Self {
        Self {
            x: 0,
            y: 0,
            enabled: false,
            index: 0,
            visible,
            count: 0,
            dimension: 3,
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
    pub fn index(&self) -> u64 {
        self.index
    }
    #[inline(always)]
    pub fn set_count(&mut self, count: u64) {
        self.count = count;
        self.index = if self.count > 0 { self.index.min(self.count - 1) } else { 0 };
        self.enabled = self.count > 0;
    }
    #[inline(always)]
    pub fn update_count(&mut self, visible_indexes: u64, total_indexes: u64) {
        if (visible_indexes <= total_indexes) && (visible_indexes != 0) {
            self.count = (total_indexes - visible_indexes) + 1;
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
        let dimension = h - (increase_margin + decrease_margin);
        self.x = if outside_rectangle { w } else { w - 1 };
        self.y = decrease_margin;
        self.dimension = dimension.max(3) as u16;
        self.visible = dimension >= 3;
        if (w < 1) || (h < 1) {
            self.visible = false;
        }
    }
    #[inline(always)]
    pub(super) fn recompute_position(&mut self, pos: i32, available_size: i32, control_size: Size) -> i32 {
        if available_size <= 4 {
            self.visible = false;
            return 0;
        }
        self.x = control_size.width as i32;
        self.y = pos;
        self.dimension = available_size as u16;
        self.visible = true;
        available_size
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
    }
    fn mouse_coords_to_scroll_pos(&self, x: i32, y: i32) -> MousePosition {
        match () {
            _ if self.x != x => MousePosition::OutsideScrollBar,
            _ if self.y == y => MousePosition::MinimizeArrow,
            _ if (self.y + (self.dimension as i32) - 1) == y => MousePosition::MaximizeArrow,
            _ if (y > self.y) && (y < (self.y + (self.dimension as i32) - 1)) => MousePosition::Bar,
            _ => MousePosition::OutsideScrollBar,
        }
    }
    fn mouse_coords_to_scroll_pos_for_dragging(&self, x: i32, y: i32) -> MousePosition {
        // we will not force x to be equal cu self.x or y to self.y

        match () {
            _ if self.y == y => MousePosition::MinimizeArrow,
            _ if (self.y + (self.dimension as i32) - 1) == y => MousePosition::MaximizeArrow,
            _ if (y > self.y) && (y < (self.y + (self.dimension as i32) - 1)) => MousePosition::Bar,
            _ => MousePosition::OutsideScrollBar,
        }
    }
    fn get_hover_status(&mut self, x: i32, y: i32) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos(x, y) {
            MousePosition::MinimizeArrow => MouseOnScrollbarStatus::HoverOnMinimizeArrow,
            MousePosition::MaximizeArrow => MouseOnScrollbarStatus::HoverOnMaximizeArrow,
            MousePosition::Bar => MouseOnScrollbarStatus::HoverOnBar,
            MousePosition::OutsideScrollBar => MouseOnScrollbarStatus::None,
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
    fn get_drag_status(&self, x: i32, y: i32) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos_for_dragging(x, y) {
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
                let poz = y - (self.y + 1);
                // sanity check & shaddowing
                let poz = poz.max(0) as u128;
                if (self.dimension > 3) && (self.count > 0) {
                    let dim = ((self.dimension as u64) - 3) as u128;
                    let cnt = (self.count - 1) as u128;
                    let mut new_index = (poz.min(dim) * cnt) / dim;
                    let rest = (poz.min(dim) * cnt) % dim;
                    if rest != 0 {
                        new_index += 1;
                    }
                    self.set_index(new_index as u64);
                    //eprintln!("new_poz={new_index},count={},dim={},y={y},self.y={}", self.count, self.dimension, self.y);
                } else {
                    self.set_index(0);
                }
            }
            _ => {}
        }
    }
    pub fn on_mouse_event(&mut self, event: &MouseEvent) -> ProcessEventResult {
        if !(self.visible && self.enabled) {
            // if scroll bar is invisible --> pass the event to control
            return ProcessEventResult::PassToControl;
        }
        match event {
            MouseEvent::Over(data) => {
                let new_status = self.get_hover_status(data.x, data.y);
                if new_status != self.status {
                    self.status = new_status;
                    if new_status.is_none() {
                        return ProcessEventResult::PassToControlAndRepaint;
                    } else {
                        return ProcessEventResult::Repaint;
                    }
                }
                ProcessEventResult::Processed
            }
            MouseEvent::Pressed(data) => {
                let new_status = self.get_press_status(data.x, data.y);
                if !new_status.is_none() {
                    self.status = new_status;
                    self.update_index_for_mouse_pos(data.x, data.y, self.status);
                    return ProcessEventResult::Update;
                }
                if self.status != MouseOnScrollbarStatus::None {
                    self.status = new_status;
                    return ProcessEventResult::PassToControlAndRepaint;
                }
                ProcessEventResult::PassToControl
            }
            MouseEvent::Released(data) => {
                let new_status = self.get_hover_status(data.x, data.y);
                if self.status.is_pressed() {
                    self.status = new_status;
                    ProcessEventResult::Repaint
                } else {
                    self.status = new_status;
                    ProcessEventResult::PassToControlAndRepaint
                }
            }
            MouseEvent::Drag(data) => {
                let new_status = self.get_drag_status(data.x, data.y);
                if self.status.is_pressed() {
                    if (new_status == self.status) && (new_status == MouseOnScrollbarStatus::PressedOnBar) {
                        self.update_index_for_mouse_pos(data.x, data.y, MouseOnScrollbarStatus::PressedOnBar);
                        ProcessEventResult::Update
                    } else {
                        ProcessEventResult::Processed
                    }
                } else {
                    ProcessEventResult::PassToControl
                }
            }
            MouseEvent::Enter => {
                self.status = MouseOnScrollbarStatus::None;
                ProcessEventResult::PassToControlAndRepaint
            }
            MouseEvent::Leave => {
                if !self.status.is_none() {
                    self.status = MouseOnScrollbarStatus::None;
                    ProcessEventResult::PassToControlAndRepaint
                } else {
                    ProcessEventResult::PassToControl
                }
            }
            MouseEvent::DoubleClick(data) => {
                let new_status = self.get_press_status(data.x, data.y);
                if !new_status.is_none() {
                    self.status = new_status;
                    self.update_index_for_mouse_pos(data.x, data.y, self.status);
                    return ProcessEventResult::Update;
                }
                if self.status != MouseOnScrollbarStatus::None {
                    self.status = new_status;
                    return ProcessEventResult::PassToControlAndRepaint;
                }
                ProcessEventResult::PassToControl
            }
            MouseEvent::Wheel(_) => ProcessEventResult::PassToControl,
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
