use crate::graphics::{ClipArea, Point, Surface};
use crate::input::Key;
use crate::system::Theme;
use crate::terminal::Terminal;

use super::events::{Control, EventProcessStatus};
use super::ControlBase;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_VERSION: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct ParentLayout {
    pub(super) clip: ClipArea,
    pub(super) origin: Point,
    pub(super) width: u16,
    pub(super) height: u16,
}
impl From<&mut ControlBase> for ParentLayout {
    fn from(base: &mut ControlBase) -> Self {
        let sz = base.get_size();
        ParentLayout {
            clip: base.get_client_clip(),
            origin: base.screen_origin,
            width: sz.width as u16,
            height: sz.height as u16,
        }
    }
}
impl From<&Box<dyn Terminal>> for ParentLayout {
    fn from(terminal: &Box<dyn Terminal>) -> Self {
        ParentLayout {
            clip: ClipArea::new(
                0,
                0,
                (terminal.get_width() as i32) - 1,
                (terminal.get_height() as i32) - 1,
            ),
            origin: Point::default(),
            width: terminal.get_width() as u16,
            height: terminal.get_height() as u16,
        }
    }
}

pub(crate) struct ControlManager {
    interface: NonNull<dyn Control>,
    base: *mut ControlBase,
    version: u32,
}
impl ControlManager {
    #[inline]
    pub(crate) fn get_control(&self) -> &dyn Control {
        unsafe { &*(self.interface.as_ptr()) }
    }
    #[inline]
    pub(crate) fn get_control_mut(&mut self) -> &mut dyn Control {
        unsafe { &mut *(self.interface.as_ptr()) }
    }
    #[inline(always)]
    pub(crate) fn get_base(&self) -> &ControlBase {
        unsafe { &*self.base }
    }
    #[inline(always)]
    pub(crate) fn get_base_mut(&mut self) -> &mut ControlBase {
        unsafe { &mut *self.base }
    }
    #[inline]
    pub(crate) fn get_version(&self) -> u32 {
        self.version
    }
    pub(crate) fn new<T>(obj: T) -> ControlManager
    where
        T: Control + 'static,
    {
        let ptr = Box::into_raw(Box::new(obj));
        let ctrl: NonNull<dyn Control> = unsafe { NonNull::new_unchecked(ptr) };
        ControlManager {
            interface: ctrl,
            base: ptr as *mut ControlBase,
            version: (GLOBAL_VERSION.fetch_add(1, Ordering::SeqCst) & 0xFFFFFFFF) as u32,
        }
    }
    pub(crate) fn get_mut<T>(&mut self) -> &mut T
    where
        T: Control + 'static,
    {
        unsafe { &mut *(self.base as *mut T) }
    }
    pub(crate) fn paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.get_base().prepare_paint(surface) {
            // paint is possible
            self.get_control().on_paint(surface, theme);
            for child in &self.get_base().children {
                child.paint(surface, theme);
            }
        }
    }
    pub(crate) fn update_layout(&mut self, parent_layout: &ParentLayout) {
        let base = self.get_base_mut();
        let old_size = base.get_size();
        base.update_control_layout_and_screen_origin(parent_layout);
        let new_size = base.get_size();
        // process the same thing for its children
        let my_layout = ParentLayout::from(base);
        // if size has been changed --> call on_resize
        if new_size!=old_size {
            self.get_control_mut().on_resize(old_size, new_size);
        }
        for child in &mut self.get_base_mut().children {
            child.update_layout(&my_layout);
        }
    }
    pub(crate) fn process_keypressed_event(
        &mut self,
        key: Key,
        character: char,
    ) -> EventProcessStatus {
        let base = self.get_base_mut();
        if base.can_receive_input() == false {
            return EventProcessStatus::Ignored;
        }
        let focused_child_index = base.focused_child_index as usize;
        if focused_child_index >= base.children.len() {
            return EventProcessStatus::Ignored;
        }
        let child = &mut base.children[focused_child_index];
        if child.process_keypressed_event(key, character) == EventProcessStatus::Processed {
            return EventProcessStatus::Processed;
        }
        // else --> call it ourselves
        return self.get_control_mut().on_key_pressed(key, character);
    }
}

impl Drop for ControlManager {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.interface.as_ptr()) };
    }
}
