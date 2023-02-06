use super::events::Event;
use super::events::{OnDefaultAction, OnKeyPressed, OnMouseEvent, OnPaint};
use super::layout::ControlLayout;
use super::{ControlWrapper, Layout};
use crate::graphics::*;
use crate::input::*;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum StatusFlags {
    Visible = 0x01,
    Enabled = 0x02,
    AcceptInput = 0x04,
}
#[derive(Copy, Clone)]
struct Margins {
    left: u8,
    right: u8,
    top: u8,
    bottom: u8,
}

#[repr(C)]
pub struct ControlManager {
    layout: ControlLayout,
    margins: Margins,
    pub(crate) children: Vec<ControlWrapper>,
    pub(crate) focused_child: u32,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
    pub(crate) hotkey: Key,
}

impl ControlManager {
    pub fn new(layout: Layout, status_flags: StatusFlags) -> Self {
        Self {
            children: Vec::new(),
            focused_child: 0,
            layout: ControlLayout::new(layout.format),
            margins: Margins {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            },
            status_flags: status_flags,
            screen_clip: ClipArea::default(),
            screen_origin: Point::default(),
            hotkey: Key::default(),
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
    pub fn can_receive_input(&self) -> bool {
        // all 3 flags must be present for an object to be able to receive input (key or mouse)
        self.status_flags
            .contains(StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput)
    }
    #[inline]
    pub fn has_focus(&self) -> bool {
        false
    }
    #[inline]
    pub fn is_mouse_over(&self) -> bool {
        false
    }
    #[inline]
    pub fn set_size_bounds(
        &mut self,
        min_width: u16,
        min_height: u16,
        max_width: u16,
        max_height: u16,
    ) {
        self.layout
            .set_size_bounds(min_width, min_height, max_width, max_height);
    }
    #[inline]
    pub fn set_hotkey(&mut self, hotkey: Key) {
        self.hotkey = hotkey
    }
    #[inline]
    pub fn get_hotkey(&self) -> Key {
        self.hotkey
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
            c.get_manager_mut().update_layout(
                &client_clip,
                self.screen_origin,
                self.layout.get_width(),
                self.layout.get_height(),
            );
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
    pub(crate) fn raise_event(&self, event: Event) {
        todo!();
    }
}
impl OnPaint for ControlManager {}
impl OnKeyPressed for ControlManager {}
impl OnMouseEvent for ControlManager {}
impl OnDefaultAction for ControlManager {}
