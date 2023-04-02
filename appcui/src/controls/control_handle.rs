use std::marker::PhantomData;

#[derive(Copy,Clone,PartialEq)]
pub struct ControlHandle<T> {
    index: u32,
    version: u32,
    _phantom: PhantomData<T>,
}
impl<T> ControlHandle<T> {
    pub (crate) fn new(index: u32, version: u32) -> Self {
        ControlHandle {
            index: index,
            version: version,
            _phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        (self.index != 0xFFFFFFFF) || (self.version != 0xFFFFFFFF)
    }
}
impl<T> Default for ControlHandle<T> {
    fn default() -> Self {
        ControlHandle {
            index: 0xFFFFFFFF,
            version: 0xFFFFFFFF,
            _phantom: PhantomData,
        }
    }
}
