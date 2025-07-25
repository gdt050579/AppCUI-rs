use std::ops::{Deref, DerefMut};

use crate::{
    prelude::{Control, NotDesktop, NotWindow},
    system::Handle,
    ui::Layout,
};

use super::ControlBase;

#[repr(C)]
#[derive(Default)]
pub struct ContainerBase {
    pub(crate) control_base: ControlBase,
}

impl ContainerBase {
    /// Creates a new container with the specified layout. The argument `accept_input` specifies if the container can receive input or not.
    pub fn new(layout: Layout, accept_input: bool) -> Self {
        Self {
            control_base: ControlBase::new(layout, accept_input),
        }
    }
    /// Creates a new container with the specified layout that has support for focused overlay.
    /// When such a container is created if it has focus it will increase its bottom and right margins by one character.
    /// This provides aditional space for the focused container to be drawn (usually a scrollbar).
    pub fn with_focus_overlay(layout: Layout) -> Self {
        Self {
            control_base: ControlBase::with_focus_overlay(layout),
        }
    }
    #[inline(always)]
    pub fn add<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.add_child(control)
    }
    // Sets the margins of the container (all childern of this control will be translated within this margins)
    #[inline]
    pub fn set_margins(&mut self, left: u8, top: u8, right: u8, bottom: u8) {
        self.margins.left = left;
        self.margins.top = top;
        self.margins.bottom = bottom;
        self.margins.right = right;
    }
}

impl Deref for ContainerBase {
    type Target = ControlBase;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.control_base
    }
}
impl DerefMut for ContainerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.control_base
    }
}
