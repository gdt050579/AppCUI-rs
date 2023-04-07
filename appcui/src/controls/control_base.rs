use super::control_manager::ParentLayout;
use super::events::{Control, Event, OnFocus};
use super::events::{OnDefaultAction, OnKeyPressed, OnMouseEvent, OnPaint, OnResize};
use super::layout::ControlLayout;
use super::{ControlHandle, ControlID, ControlManager, Layout};
use crate::graphics::*;
use crate::input::*;
use crate::system::RuntimeManager;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub enum StatusFlags {
    Visible = 0x01,
    Enabled = 0x02,
    AcceptInput = 0x04,
    Focused = 0x08,
}
#[derive(Copy, Clone)]
struct Margins {
    left: u8,
    right: u8,
    top: u8,
    bottom: u8,
}

#[repr(C)]
pub struct ControlBase {
    layout: ControlLayout,
    margins: Margins,
    parent: Option<*mut ControlBase>,
    pub(crate) children: Vec<Option<ControlManager>>,
    pub(crate) focused_child_index: u32,
    status_flags: StatusFlags,
    pub(crate) screen_clip: ClipArea,
    pub(crate) screen_origin: Point,
    pub(crate) hotkey: Key,
    pub(crate) id: ControlID,
}

impl ControlBase {
    pub fn new(layout: Layout, status_flags: StatusFlags) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            focused_child_index: 0,
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
            id: ControlID::new(),
        }
    }
    #[inline(always)]
    pub fn get_size(&self) -> Size {
        Size {
            width: self.layout.get_width() as u32,
            height: self.layout.get_height() as u32,
        }
    }
    #[inline(always)]
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.layout.resize(width, height);
    }

    #[inline(always)]
    pub fn get_position(&self) -> Point {
        Point {
            x: self.layout.get_x() as i32,
            y: self.layout.get_y() as i32,
        }
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.layout.set_position(x, y);
        //GDT: we should change the layout mode
        RuntimeManager::get().request_recompute_layout();
    }

    pub fn request_focus(&mut self) -> bool {
        if self.has_focus() || !self.can_receive_input() {
            return false;
        }
        // we need to check if current child can receive focus
        // if yes, we shoudl request focus for it
        // if no, just request focus for the current control

        RuntimeManager::get().request_focus_for_control(self.id);
        true
    }

    #[inline]
    pub(crate) fn get_parent_mut(&mut self) -> Option<&mut ControlBase> {
        unsafe {
            if let Some(p) = self.parent {
                Some(&mut *p)
            } else {
                None
            }
        }
    }
    pub(crate) fn add_child<T>(&mut self, control: T) -> ControlHandle<T>
    where
        T: Control + 'static,
    {
        let mut c = ControlManager::new(control);
        c.get_base_mut().parent = Some(self as *mut ControlBase);
        let idx = self.children.len() as u32;
        let id = c.get_base().id;
        self.children.push(Some(c));
        // if no control is focused, we should test to see if the current one can be focused
        return ControlHandle::<T>::new(idx, id);
    }

    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        self.status_flags.contains(StatusFlags::Visible)
    }
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.status_flags.contains(StatusFlags::Enabled)
    }
    #[inline(always)]
    pub fn can_receive_input(&self) -> bool {
        // all 3 flags must be present for an object to be able to receive input (key or mouse)
        self.status_flags
            .contains(StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput)
    }
    #[inline(always)]
    pub fn has_focus(&self) -> bool {
        self.status_flags.contains(StatusFlags::Focused)
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
    pub(crate) fn set_margins(&mut self, left: u8, top: u8, right: u8, bottom: u8) {
        self.margins.left = left;
        self.margins.top = top;
        self.margins.bottom = bottom;
        self.margins.right = right;
    }
    #[inline]
    pub fn set_hotkey(&mut self, hotkey: Key) {
        self.hotkey = hotkey
    }
    #[inline]
    pub fn get_hotkey(&self) -> Key {
        self.hotkey
    }
    #[inline(always)]
    pub(crate) fn is_coord_in_control(&self, x: i32, y: i32) -> bool {
        (x >= 0)
            && (y >= 0)
            && (x < (self.layout.get_width() as i32))
            && (y < (self.layout.get_height() as i32))
    }

    #[inline]
    pub(crate) fn update_control_layout_and_screen_origin(&mut self, parent_layout: &ParentLayout) {
        self.layout
            .update(parent_layout.width, parent_layout.height);
        self.screen_origin.x = parent_layout.origin.x + self.layout.get_x();
        self.screen_origin.y = parent_layout.origin.y + self.layout.get_y();
        self.screen_clip.set_with_size(
            self.screen_origin.x,
            self.screen_origin.y,
            self.layout.get_width(),
            self.layout.get_height(),
        );
        self.screen_clip.intersect_with(&parent_layout.clip);
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
    pub(crate) fn raise_event(&self, _event: Event) {
        todo!();
    }
    pub(crate) fn show_tooltip_on_point(&self, txt: &str, x: i32, y: i32) {
        if self.is_visible() && self.screen_clip.is_visible() {
            let r = Rect::with_size(self.screen_clip.left + x, self.screen_clip.top + y, 1, 1);
            RuntimeManager::get().show_tooltip(txt, &r);
        } else {
            RuntimeManager::get().hide_tooltip();
        }
    }
    pub(crate) fn show_tooltip(&self, txt: &str) {
        if self.is_visible() && self.screen_clip.is_visible() {
            let r = Rect::new(
                self.screen_clip.left,
                self.screen_clip.top,
                self.screen_clip.right,
                self.screen_clip.bottom,
            );
            RuntimeManager::get().show_tooltip(txt, &r);
        } else {
            RuntimeManager::get().hide_tooltip();
        }
    }
    pub(crate) fn hide_tooltip(&self) {
        RuntimeManager::get().hide_tooltip();
    }
}
impl OnPaint for ControlBase {}
impl OnKeyPressed for ControlBase {}
impl OnMouseEvent for ControlBase {}
impl OnDefaultAction for ControlBase {}
impl OnResize for ControlBase {}
impl OnFocus for ControlBase {}
