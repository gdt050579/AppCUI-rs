use super::ProcessEventResult;
use crate::prelude::MouseEvent;

#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum MouseOnScrollbarStatus {
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
    pub(crate) fn is_none(&self) -> bool {
        *self == MouseOnScrollbarStatus::None
    }
    #[inline(always)]
    pub(crate) fn is_pressed(&self) -> bool {
        matches!(
            self,
            MouseOnScrollbarStatus::PressedOnMinimizeArrow | MouseOnScrollbarStatus::PressedOnMaximizeArrow | MouseOnScrollbarStatus::PressedOnBar
        )
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub(super) enum MousePosition {
    MinimizeArrow,
    MaximizeArrow,
    Bar,
    OutsideScrollBar,
}

pub struct GenericScrollBar {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) dimension: u16,
    pub(crate) enabled: bool,
    pub(crate) visible: bool,
    pub(crate) max_value: u64,
    pub(crate) value: u64,
    pub(crate) status: MouseOnScrollbarStatus,
}
impl GenericScrollBar {
    pub fn new(visible: bool) -> Self {
        Self {
            x: 0,
            y: 0,
            enabled: false,
            value: 0,
            visible,
            max_value: 0,
            dimension: 3,
            status: MouseOnScrollbarStatus::None,
        }
    }
    #[inline(always)]
    pub fn set_value(&mut self, value: u64) {
        self.value = value.min(self.max_value);
    }
    #[inline(always)]
    pub fn value(&self) -> u64 {
        self.value
    }
    #[inline(always)]
    pub fn update(&mut self, visible_values: u64, available_values: u64) {
        if (visible_values < available_values) && (visible_values != 0) {
            self.max_value = available_values - visible_values;
        } else {
            self.max_value = 0;
        }
        self.value = self.value.min(self.max_value);
        self.enabled = self.max_value > 0;
    }
    #[inline(always)]
    pub(super) fn recompute_layout(&mut self, x: i32, y: i32, available_space: u16) -> u16 {
        if available_space <= 4 {
            self.visible = false;
            return 0;
        }
        self.y = y;
        self.x = x;
        self.dimension = available_space;
        self.visible = true;
        available_space
    }
    #[inline(always)]
    pub(super) fn update_value_from_mouse(&mut self, mouse_pos: i32, new_status: MouseOnScrollbarStatus) {
        match new_status {
            MouseOnScrollbarStatus::PressedOnMinimizeArrow => {
                if self.value > 0 {
                    self.set_value(self.value - 1);
                }
            }
            MouseOnScrollbarStatus::PressedOnMaximizeArrow => {
                self.set_value(self.value + 1);
            }
            MouseOnScrollbarStatus::PressedOnBar => {
                // sanity check & shaddowing
                let poz = mouse_pos.max(0) as u128;
                if (self.dimension > 3) && (self.max_value > 0) {
                    let dim = ((self.dimension as u64) - 3) as u128;
                    let cnt = self.max_value as u128;
                    let mut new_index = (poz.min(dim) * cnt) / dim;
                    let rest = (poz.min(dim) * cnt) % dim;
                    if rest != 0 {
                        new_index += 1;
                    }
                    self.set_value(new_index as u64);
                    //eprintln!("new_poz={new_index},count={},dim={},y={y},self.y={}", self.count, self.dimension, self.y);
                } else {
                    self.set_value(0);
                }
            }
            _ => {}
        }
    }
    #[inline(always)]
    pub(super) fn value_to_screen_offset(&self) -> i32 {
        if (self.max_value == 0) || (self.dimension <= 3) {
            return 0;
        }
        let dim = (self.dimension as u64) - 3;
        let cnt = self.max_value;
        let idx = self.value.min(cnt); // safety check
        if self.max_value > 0x0000_0FFF_FFFF_FFFFu64 {
            ((((idx as u128) * (dim as u128)) / (cnt as u128)) as u16) as i32
        } else {
            (((idx * dim) / cnt) as u16) as i32
        }
    }

    #[inline(always)]
    fn mouse_coords_to_scroll_pos(&self, x: i32, y: i32, vertical: bool) -> MousePosition {
        if vertical {
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
    #[inline(always)]
    fn mouse_coords_to_scroll_pos_for_dragging(&self, x: i32, y: i32, vertical: bool) -> MousePosition {
        // we will not force x to be equal cu self.x or y to self.y
        if vertical {
            match () {
                _ if self.y == y => MousePosition::MinimizeArrow,
                _ if (self.y + (self.dimension as i32) - 1) == y => MousePosition::MaximizeArrow,
                _ if (y > self.y) && (y < (self.y + (self.dimension as i32) - 1)) => MousePosition::Bar,
                _ => MousePosition::OutsideScrollBar,
            }
        } else {
            match () {
                _ if self.x == x => MousePosition::MinimizeArrow,
                _ if (self.x + (self.dimension as i32) - 1) == x => MousePosition::MaximizeArrow,
                _ if (x > self.x) && (x < (self.x + (self.dimension as i32) - 1)) => MousePosition::Bar,
                _ => MousePosition::OutsideScrollBar,
            }
        }
    }
    fn get_hover_status(&mut self, x: i32, y: i32, vertical: bool) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos(x, y, vertical) {
            MousePosition::MinimizeArrow => MouseOnScrollbarStatus::HoverOnMinimizeArrow,
            MousePosition::MaximizeArrow => MouseOnScrollbarStatus::HoverOnMaximizeArrow,
            MousePosition::Bar => MouseOnScrollbarStatus::HoverOnBar,
            MousePosition::OutsideScrollBar => MouseOnScrollbarStatus::None,
        }
    }
    fn get_press_status(&self, x: i32, y: i32, vertical: bool) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos(x, y, vertical) {
            MousePosition::MinimizeArrow => MouseOnScrollbarStatus::PressedOnMinimizeArrow,
            MousePosition::MaximizeArrow => MouseOnScrollbarStatus::PressedOnMaximizeArrow,
            MousePosition::Bar => MouseOnScrollbarStatus::PressedOnBar,
            MousePosition::OutsideScrollBar => MouseOnScrollbarStatus::None,
        }
    }
    fn get_drag_status(&self, x: i32, y: i32, vertical: bool) -> MouseOnScrollbarStatus {
        match self.mouse_coords_to_scroll_pos_for_dragging(x, y, vertical) {
            MousePosition::MinimizeArrow => MouseOnScrollbarStatus::PressedOnMinimizeArrow,
            MousePosition::MaximizeArrow => MouseOnScrollbarStatus::PressedOnMaximizeArrow,
            MousePosition::Bar => MouseOnScrollbarStatus::PressedOnBar,
            MousePosition::OutsideScrollBar => MouseOnScrollbarStatus::None,
        }
    }
    pub fn process_mouse_event(&mut self, event: &MouseEvent, vertical: bool) -> ProcessEventResult {
        if !(self.visible && self.enabled) {
            // if scroll bar is invisible --> pass the event to control
            return ProcessEventResult::PassToControl;
        }
        match event {
            MouseEvent::Over(data) => {
                let new_status = self.get_hover_status(data.x, data.y, vertical);
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
                let new_status = self.get_press_status(data.x, data.y, vertical);
                if !new_status.is_none() {
                    let dif = if vertical { data.y - (self.y + 1) } else { data.x - (self.x + 1) };
                    self.status = new_status;
                    //print!("Status:{:?}, max={}, value={}, dif={} ",new_status,self.max_value,self.value,dif);
                    self.update_value_from_mouse(dif, new_status);
                    //println!("=> value={}",self.value);
                    return ProcessEventResult::Update;
                }
                if self.status != MouseOnScrollbarStatus::None {
                    self.status = new_status;
                    return ProcessEventResult::PassToControlAndRepaint;
                }
                ProcessEventResult::PassToControl
            }
            MouseEvent::Released(data) => {
                let new_status = self.get_hover_status(data.x, data.y, vertical);
                if self.status.is_pressed() {
                    self.status = new_status;
                    ProcessEventResult::Repaint
                } else {
                    self.status = new_status;
                    ProcessEventResult::PassToControlAndRepaint
                }
            }
            MouseEvent::Drag(data) => {
                let new_status = self.get_drag_status(data.x, data.y, vertical);
                if self.status.is_pressed() {
                    if (new_status == self.status) && (new_status == MouseOnScrollbarStatus::PressedOnBar) {
                        let dif = if vertical { data.y - (self.y + 1) } else { data.x - (self.x + 1) };
                        self.update_value_from_mouse(dif, MouseOnScrollbarStatus::PressedOnBar);
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
                let new_status = self.get_press_status(data.x, data.y, vertical);
                if !new_status.is_none() {
                    let dif = if vertical { data.y - (self.y + 1) } else { data.x - (self.x + 1) };
                    self.status = new_status;
                    self.update_value_from_mouse(dif, new_status);
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
}
