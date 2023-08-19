use crate::{system::Handle, ui::common::UIElement};
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq)]
pub struct MenuHandle {
    pub (crate) handle: Handle<UIElement>,
}
impl MenuHandle {
    #[allow(non_upper_case_globals)]
    pub const None: MenuHandle = MenuHandle { handle: Handle::None };
    pub (crate) fn new(index: u32) -> Self {
        Self {
            handle: Handle::new(index),
        }
    }
}
impl Deref for MenuHandle {
    type Target = Handle<UIElement>;
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
