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

pub struct GenericScrollBar {
    x: i32,
    y: i32,
    dimension: u16,
    enabled: bool,
    visible: bool,
    count: u64,
    index: u64,
    status: MouseOnScrollbarStatus,
}
impl GenericScrollBar {
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
    pub fn set_index(&mut self, value: u64) {
        self.index = if self.count > 0 { value.min(self.count - 1) } else { 0 };
    }
    #[inline(always)]
    pub fn index(&self) -> u64 {
        self.index
    }
    #[inline(always)]
    pub fn update(&mut self, visible_indexes: u64, total_indexes: u64) {
        if (visible_indexes < total_indexes) && (visible_indexes != 0) {
            self.count = (total_indexes - visible_indexes) + 1;
        } else {
            self.count = 0;
        }
        self.index = if self.count > 0 { self.index.min(self.count - 1) } else { 0 };
        self.enabled = self.count > 0;
    }
    #[inline(always)]
    pub(super) fn recompute_layout(&mut self, pos: i32, available_size: i32, control_size: Size) -> i32 {
        if available_size <= 4 {
            self.visible = false;
            return 0;
        }
        self.y = control_size.height as i32;
        self.x = pos;
        self.dimension = available_size as u16;
        self.visible = true;
        available_size
    }
    fn update_index_for_mouse_pos(&mut self, x: i32, new_status: MouseOnScrollbarStatus) {
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
                let poz = x - (self.x + 1);
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
