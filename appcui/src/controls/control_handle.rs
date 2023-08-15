use std::marker::PhantomData;

use crate::system::Handle;

#[derive(PartialEq)]
pub struct ControlHandle<T> {
    handle: Handle,
    _phantom: PhantomData<T>,
}
impl<T> ControlHandle<T> {
    #[allow(non_upper_case_globals)]
    pub const None: ControlHandle<T> = ControlHandle {
        handle: Handle::None,
        _phantom: PhantomData,
    };
    pub(crate) fn new(handle: Handle) -> Self {
        ControlHandle {
            handle,
            _phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub(crate) fn get_index(&self) -> usize {
        self.handle.get_index()
    }
    #[inline(always)]
    pub(crate) fn get_handle(&self) -> Handle {
        self.handle
    }
}
impl<T> Clone for ControlHandle<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            _phantom: self._phantom.clone(),
        }
    }
}
impl<T> Copy for ControlHandle<T> {}
impl<T> PartialEq<Handle> for ControlHandle<T> {
    fn eq(&self, other: &Handle) -> bool {
        self.handle == *other
    }
}
