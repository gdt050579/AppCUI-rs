use std::marker::PhantomData;

use super::ControlID;

#[derive(Copy, Clone, PartialEq)]
pub struct ControlHandle<T> {
    index: u32,
    id: ControlID,
    _phantom: PhantomData<T>,
}
impl<T> ControlHandle<T> {
    pub(crate) fn new(index: u32, id: ControlID) -> Self {
        ControlHandle {
            index,
            id,
            _phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        (self.index != 0xFFFFFFFF) || (self.id.is_valid())
    }
}
impl<T> Default for ControlHandle<T> {
    fn default() -> Self {
        ControlHandle {
            index: 0xFFFFFFFF,
            id: ControlID::INVALID,
            _phantom: PhantomData,
        }
    }
}
