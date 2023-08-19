use std::marker::PhantomData;

use crate::{system::Handle, ui::common::UIElement};

#[derive(PartialEq)]
pub struct ToolBarItemHandle<T> {
    pub (crate) handle: Handle<UIElement>,
    _phantom: PhantomData<T>
}

impl<T> ToolBarItemHandle<T> {
    #[allow(non_upper_case_globals)]
    pub const None: ToolBarItemHandle<T> = ToolBarItemHandle {
        handle: Handle::None,
        _phantom: PhantomData,
    };
    pub (crate) fn new(handle: Handle<UIElement>)->Self {
        Self {
            handle,
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for ToolBarItemHandle<T> {
    fn clone(&self) -> Self {
        Self { handle: self.handle.clone(), _phantom: self._phantom.clone() }
    }
}
impl<T> Copy for ToolBarItemHandle<T> {

}