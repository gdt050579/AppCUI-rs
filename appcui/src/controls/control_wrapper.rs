use super::events::Control;
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
    #[inline]
    pub(crate) fn get_manager(&self) -> &ControlManager {
        unsafe { &*self.manager }
    }
    #[inline]
    pub(crate) fn get_manager_mut(&mut self) -> &mut ControlManager {
        unsafe { &mut *self.manager }
    }
    #[inline]
    pub(crate) fn get_version(&self) -> u32 {
       self.version
    }
    pub(crate) fn new<T>(obj: T) -> ControlWrapper
    where
        T: Control +'static,
    {
        let ptr = Box::into_raw(Box::new(obj));
        let ctrl: NonNull<dyn Control> = unsafe { NonNull::new_unchecked(ptr) };
        ControlWrapper {
            interface: ctrl,
            manager: ptr as *mut ControlManager,
            version: (GLOBAL_VERSION.fetch_add(1, Ordering::SeqCst) & 0xFFFFFFFF) as u32,
        }
    }
}

impl Drop for ControlWrapper {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.interface.as_ptr()) };
    }
}