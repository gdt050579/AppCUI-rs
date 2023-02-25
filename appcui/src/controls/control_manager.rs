use crate::graphics::{ClipArea, Point, Surface};
use crate::input::Key;
use crate::system::Theme;

use super::events::{Control, EventProcessStatus};
use super::ControlBase;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_VERSION: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct ControlManager {
    interface: NonNull<dyn Control>,
    base: *mut ControlBase,
    version: u32,
    index: u32,
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
            index: ControlBase::INVALID_CONTROL_ID,
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
    pub(crate) fn update_layout(
        &mut self,
        parent_clip: &ClipArea,
        parent_origin: Point,
        parent_width: u16,
        parent_height: u16,
    ) {
        let base = self.get_base_mut();
        base.update_control_layout_and_screen_origin(
            parent_clip,
            parent_origin,
            parent_width,
            parent_height,
        );
        // process the same thing for its children
        let client_clip = base.get_client_clip();
        let w = base.get_width();
        let h = base.get_height();
        let p = base.screen_origin;
        for child in &mut base.children {
            child.update_layout(&client_clip, p, w, h);
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
