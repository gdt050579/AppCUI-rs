use crate::graphics::Surface;
use crate::input::Key;
use crate::system::Theme;

use super::events::{Control, KeyPressedResult};
use super::ControlManager;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_VERSION: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct ControlWrapper {
    interface: NonNull<dyn Control>,
    manager: *mut ControlManager,
    version: u32,
}
impl ControlWrapper {
    #[inline]
    pub(crate) fn get_control(&self) -> &dyn Control {
        unsafe { &*(self.interface.as_ptr()) }
    }
    #[inline]
    pub(crate) fn get_control_mut(&mut self) -> &mut dyn Control {
        unsafe { &mut *(self.interface.as_ptr()) }
    }
    #[inline(always)]
    pub(crate) fn get_manager(&self) -> &ControlManager {
        unsafe { &*self.manager }
    }
    #[inline(always)]
    pub(crate) fn get_manager_mut(&mut self) -> &mut ControlManager {
        unsafe { &mut *self.manager }
    }
    #[inline]
    pub(crate) fn get_version(&self) -> u32 {
        self.version
    }
    pub(crate) fn new<T>(obj: T) -> ControlWrapper
    where
        T: Control + 'static,
    {
        let ptr = Box::into_raw(Box::new(obj));
        let ctrl: NonNull<dyn Control> = unsafe { NonNull::new_unchecked(ptr) };
        ControlWrapper {
            interface: ctrl,
            manager: ptr as *mut ControlManager,
            version: (GLOBAL_VERSION.fetch_add(1, Ordering::SeqCst) & 0xFFFFFFFF) as u32,
        }
    }
    pub(crate) fn process_keypressed_event(&mut self, key: Key, character: char) -> bool {
        let manager = self.get_manager_mut();
        let idx = manager.focused_child as usize;
        if idx < manager.children.len() {
            let focused_child = &mut manager.children[idx];
            let focused_child_manager = focused_child.get_manager_mut();
            if focused_child_manager.can_receive_input() && focused_child.process_keypressed_event(key, character) {
                return true;        
            }
        }
        // if the child did not process the key, try to process-it myself
        return self.get_control_mut().on_key_pressed(key, character) == KeyPressedResult::Processed;
    }
    pub (crate) fn paint(&mut self, surface: &mut Surface, theme: &Theme) {
        self.get_manager().prepare_paint(surface);
        self.get_control().on_paint(surface, theme);
        // paint all children
        // should be painted in a specific order
        for c in self.get_manager_mut().children.iter_mut() {
            c.paint(surface, theme);
        }
    }
}

impl Drop for ControlWrapper {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.interface.as_ptr()) };
    }
}
