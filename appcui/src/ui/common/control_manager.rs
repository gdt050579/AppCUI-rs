use super::ControlBase;
use crate::{
    graphics::{ClipArea, Point},
    terminals::Terminal,
    ui::common::traits::Control,
};
use std::ptr::NonNull;

pub(crate) struct ParentLayout {
    pub(super) clip: ClipArea,
    pub(super) origin: Point,
    pub(super) client_width: u16,
    pub(super) client_height: u16,
}
impl From<&mut ControlBase> for ParentLayout {
    fn from(base: &mut ControlBase) -> Self {
        let client_sz = base.get_client_size();
        let mut pl = ParentLayout {
            clip: base.get_client_clip(),
            origin: base.screen_origin,
            client_width: client_sz.width as u16,
            client_height: client_sz.height as u16,
        };
        pl.origin.x += base.margins.left as i32;
        pl.origin.y += base.margins.top as i32;
        pl
    }
}
impl From<&Box<dyn Terminal>> for ParentLayout {
    fn from(terminal: &Box<dyn Terminal>) -> Self {
        let term_width = terminal.get_width();
        let term_height = terminal.get_height();
        ParentLayout {
            clip: ClipArea::new(0, 0, (term_width as i32) - 1, (term_height as i32) - 1),
            origin: Point::default(),
            client_width: term_width as u16,
            client_height: term_height as u16,
        }
    }
}

pub(crate) struct ControlManager {
    interface: NonNull<dyn Control>,
    base: *mut ControlBase,
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
        unsafe { Box::from_raw(self.interface.as_ptr()) };
    }
}
