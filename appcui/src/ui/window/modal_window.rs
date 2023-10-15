use super::{Flags, Window};
use crate::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct ModalWindow<T: Sized> {
    base: Window,
    result: Option<T>,
}
impl<T> Deref for ModalWindow<T> {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T> DerefMut for ModalWindow<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl<T> ModalWindow<T> {
    pub fn new(title: &str, layout: Layout, flags: Flags) -> Self {
        Self {
            base: Window::new(title, layout, flags),
            result: None,
        }
    }
    pub fn show(mut self) -> Option<T> {
        // simple flag to make sure that you can only run this one time
        // if self.modal_loop_executed {
        //     return None;
        // }
        // self.modal_loop_executed = true;
        // // run the loop in the runtime
        // let rm = RuntimeManager::get();
        // let h = rm.add_window(self);
        None
    }
    pub fn exit_modal_window(&mut self, result: T) {
        self.result = Some(result);
    }
}
