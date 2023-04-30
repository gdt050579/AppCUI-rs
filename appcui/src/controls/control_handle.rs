use std::marker::PhantomData;

use crate::system::Handle;

#[derive(Copy, Clone, PartialEq)]
pub struct ControlHandle<T> {
    handle: Handle,
    _phantom: PhantomData<T>,
}
impl<T> ControlHandle<T> {
    pub(crate) fn new(handle: Handle) -> Self {
        ControlHandle {
            handle,
            _phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub (crate) fn get_index(&self)->usize {
        self.handle.get_index()
    }
    #[inline(always)]
    pub (crate) fn get_id(&self)->u32 {
        self.handle.get_id()
    }
}