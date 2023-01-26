use super::events::Control;
use super::ControlManager;
use std::ptr::NonNull;

pub(crate) struct ControlWrapper {
    pub(crate) interface: NonNull<dyn Control>,
    pub(crate) manager: *mut ControlManager,
    pub(crate) version: u32,
}
impl ControlWrapper {
    #[inline]
    pub(crate) fn get_control(&self) -> &dyn Control {
        unsafe { &*(self.interface.as_ptr()) }
    }
    pub(crate) fn new<T>(obj: T, version: u32) -> ControlWrapper
    where
        T: Control,
    {
        let ptr = Box::into_raw(Box::new(obj));
        let ctrl: NonNull<dyn Control> = unsafe { NonNull::new_unchecked(ptr) };
        ControlWrapper {
            interface: ctrl,
            manager: ptr as *mut ControlManager,
            version: version,
        }
    }
}
