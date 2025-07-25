use std::ops::{Deref, DerefMut};

use crate::{
    prelude::{Control, NotDesktop, NotWindow},
    system::Handle,
};

use super::ControlBase;

#[repr(C)]
#[derive(Default)]
pub struct ContainerBase {
    pub(crate) control_base: ControlBase,
}

impl ContainerBase {
    #[inline(always)]
    pub fn add<T>(&mut self, control: T) -> Handle<T>
    where
        T: Control + NotWindow + NotDesktop + 'static,
    {
        self.add_child(control)
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
