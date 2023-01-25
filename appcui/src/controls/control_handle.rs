use std::marker::PhantomData;

pub struct ControlHandle<T> {
    index: u32,
    version: u32,
    _phantom: PhantomData<T>,
}
impl<T> ControlHandle<T> {
    pub (super) fn new(index: u32, version: u32) -> Self {
        ControlHandle {
            index: index,
            version: version,
            _phantom: PhantomData,
        }
    }
}
