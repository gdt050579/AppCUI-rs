use crate::system::Handle;
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq)]
pub struct MenuHandle {
    handle: Handle,
}
impl MenuHandle {
    pub fn new(index: u32) -> Self {
        Self {
            handle: Handle::new(index),
        }
    }
}
impl Deref for MenuHandle {
    type Target = Handle;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
impl Default for MenuHandle {
    fn default() -> Self {
        Self {
            handle: Handle::default(),
        }
    }
}
