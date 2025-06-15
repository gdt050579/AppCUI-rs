use super::ControlBase;
use crate::{
    graphics::{ClipArea, Point},
    prelude::RuntimeManager,
    system::{Handle, HandleSupport},
    backend::Backend,
    ui::common::traits::Control,
};
use std::ptr::NonNull;

pub(crate) struct ParentLayout {
    pub(super) clip: ClipArea,
    pub(super) origin: Point,
    pub(super) client_width: u16,
    pub(super) client_height: u16,
}
impl From<&ControlBase> for ParentLayout {
    fn from(base: &ControlBase) -> Self {
        let client_sz = base.client_size();
        let mut pl = ParentLayout {
            clip: base.client_clip(),
            origin: base.screen_origin,
            client_width: client_sz.width as u16,
            client_height: client_sz.height as u16,
        };
        pl.origin.x += base.margins.left as i32;
        pl.origin.y += base.margins.top as i32;
        pl
    }
}
impl From<&Box<dyn Backend>> for ParentLayout {
    fn from(terminal: &Box<dyn Backend>) -> Self {
        let sz = terminal.get_size();
        ParentLayout {
            clip: ClipArea::new(0, 0, (sz.width as i32) - 1, (sz.height as i32) - 1),
            origin: Point::default(),
            client_width: sz.width as u16,
            client_height: sz.height as u16,
        }
    }
}

pub(crate) struct ControlManager {
    interface: NonNull<dyn Control>,
    base: *mut ControlBase,
}
impl ControlManager {
    #[inline]
    pub(crate) fn control(&self) -> &dyn Control {
        unsafe { &*(self.interface.as_ptr()) }
    }
    #[inline]
    pub(crate) fn control_mut(&mut self) -> &mut dyn Control {
        unsafe { &mut *(self.interface.as_ptr()) }
    }
    #[inline(always)]
    pub(crate) fn base(&self) -> &ControlBase {
        unsafe { &*self.base }
    }
    #[inline(always)]
    pub(crate) fn base_mut(&mut self) -> &mut ControlBase {
        unsafe { &mut *self.base }
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
        }
    }
    pub(crate) fn get_mut<T>(&mut self) -> &mut T
    where
        T: Control + 'static,
    {
        unsafe { &mut *(self.base as *mut T) }
    }
    pub(crate) fn get<T>(&self) -> &T
    where
        T: Control + 'static,
    {
        unsafe { &*(self.base as *const T) }
    }
}

impl Drop for ControlManager {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.interface.as_ptr());
        };
    }
}

impl HandleSupport<ControlManager> for ControlManager {
    fn handle(&self) -> Handle<ControlManager> {
        self.base().handle.cast()
    }

    fn set_handle(&mut self, handle: Handle<ControlManager>) {
        // set the handle for all children - only for non desktop controls
        if !self.base().is_desktop_control() {
            let controls = RuntimeManager::get().get_controls_mut();
            for child in self.base().children.iter() {
                if let Some(control) = controls.get_mut(*child) {
                    control.base_mut().parent = handle.cast();
                }
            }
        }
        // set my handle
        self.base_mut().handle = handle.cast();
    }
}
