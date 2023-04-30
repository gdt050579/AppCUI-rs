use std::ops::Deref;
use crate::system::Handle;

#[derive(Copy, Clone, PartialEq)]
pub struct MenuHandle {
    handle: Handle
}
impl MenuHandle {
    pub fn new(index: u32) -> Self {
        Self {
            handle: Handle::new(index)
        }
    }
}
impl Deref for MenuHandle {
    type Target = Handle;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}