use std::marker::PhantomData;

use crate::system::Handle;

#[derive(PartialEq)]
pub struct ToolBarItemHandle<T> {
    handle: Handle,
    _phantom: PhantomData<T>
}

impl<T> ToolBarItemHandle<T> {
    #[allow(non_upper_case_globals)]
    pub const None: ToolBarItemHandle<T> = ToolBarItemHandle {
        handle: Handle::None,
        _phantom: PhantomData,
    };
    pub (crate) fn new(handle: Handle)->Self {
        Self {
            handle,
            _phantom: PhantomData,
        }
    }
}