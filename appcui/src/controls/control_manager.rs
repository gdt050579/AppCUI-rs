use std::ptr::NonNull;

use super::events::{Control, OnKeyPressed, OnPaint};
use super::layout::ControlLayout;
use super::Layout;
use crate::graphics::*;
use crate::system::Theme;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum StatusFlags {
    Visible = 0x01,
    Enabled = 0x02,
    Focusable = 0x04,
}
#[derive(Copy, Clone)]
struct Margins {
    left: u8,
    right: u8,
    top: u8,
    bottom: u8,
}

struct ControlWrapper {
    interface: NonNull<dyn Control>,
    manager: *mut ControlManager,
    version: u32,
}
impl ControlWrapper {
    #[inline]
    fn get_control(&self) -> &dyn Control {
        unsafe { &*(self.interface.as_ptr()) }
    }
}
pub struct ControlManager {
    layout: ControlLayout,
    margins: Margins,
    pub(crate) children: Vec<ControlWrapper>,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
}

impl ControlManager {
    pub fn new(layout_format: Layout, status_flags: StatusFlags) -> Self {
        Self {
            children: Vec::new(),
            layout: ControlLayout::new(layout_format.format),
            margins: Margins {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
            status_flags: status_flags,
            screen_clip: ClipArea::default(),
            screen_origin: Point::default(),
        }
    }
    #[inline]
    pub fn get_width(&self) -> u16 {
        self.layout.get_width()
    }
    #[inline]
    pub fn get_height(&self) -> u16 {
        self.layout.get_height()
    }
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.status_flags.contains(StatusFlags::Visible)
    }
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.status_flags.contains(StatusFlags::Enabled)
    }

    #[inline]
    pub(crate) fn update_control_layout_and_screen_origin(
        &mut self,
        parent_clip: &ClipArea,
        parent_screen_origin: Point,
        parent_width: u16,
        parent_height: u16,
    ) {
        self.layout.update(parent_width, parent_height);
        self.screen_origin.x = parent_screen_origin.x + self.layout.get_x();
        self.screen_origin.y = parent_screen_origin.y + self.layout.get_y();
        self.screen_clip.set_with_size(
            self.screen_origin.x,
            self.screen_origin.y,
            self.layout.get_width(),
            self.layout.get_height(),
        );
        self.screen_clip.intersect_with(parent_clip);
    }
    #[inline]
    pub(crate) fn get_client_clip(&self) -> ClipArea {
        let mut c = ClipArea::with_size(
            self.screen_origin.x,
            self.screen_origin.y,
            self.layout.get_width(),
            self.layout.get_height(),
        );
        c.reduce_margins(
            self.margins.left as i32,
            self.margins.top as i32,
            self.margins.right as i32,
            self.margins.bottom as i32,
        );
        c.intersect_with(&self.screen_clip);
        c
    }

    pub(crate) fn update_layout(
        &mut self,
        parent_clip: &ClipArea,
        parent_origin: Point,
        parent_width: u16,
        parent_height: u16,
    ) {
        self.update_control_layout_and_screen_origin(
            parent_clip,
            parent_origin,
            parent_width,
            parent_height,
        );
        // process the same thing for its children
        let client_clip = self.get_client_clip();
        for c in self.children.iter_mut() {
            unsafe {
                (*c.manager).update_layout(
                    &client_clip,
                    self.screen_origin,
                    self.layout.get_width(),
                    self.layout.get_height(),
                );
            }
        }
    }
    pub(crate) fn prepare_paint(&self, surface: &mut Surface) -> bool {
        if (self.is_visible() == false) || (self.screen_clip.is_visible() == false) {
            return false; // nothing to draw
        }
        // paint myself
        surface.set_base_clip(
            self.screen_clip.left,
            self.screen_clip.top,
            self.screen_clip.right,
            self.screen_clip.bottom,
        );
        surface.set_base_origin(self.screen_origin.x, self.screen_origin.y);
        surface.reset_clip();
        surface.reset_origin();
        return true;
    }
}
impl OnPaint for ControlManager {}
impl OnKeyPressed for ControlManager {}

impl Control for BasicControl {
    fn get_basic_control(&self) -> &BasicControl {
        self
    }
    fn get_mut_basic_control(&mut self) -> &mut BasicControl {
        self
    }
}
