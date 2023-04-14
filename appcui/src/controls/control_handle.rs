use std::{marker::PhantomData, num::NonZeroU64};
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Handle {
    value: NonZeroU64,
}

impl Handle {
    const INVALID_INDEX: u32 = u32::MAX;
    pub (crate) fn new(index: u32)->Self {
        let mut id = 0u32;
        while id == 0 {
            id = GLOBAL_ID.fetch_add(1, Ordering::SeqCst) as u32;
        }
        Self { 
            value: NonZeroU64::new((index as u64) | ((id as u64)<<32)).unwrap()
        }
    }
    #[inline(always)]
    pub (crate) fn get_index(&self)->usize {
        (self.value.get() & 0xFFFFFFFF) as usize
    }
    #[inline(always)]
    pub (crate) fn get_id(&self)->u32 {
        ((self.value.get() >> 32) & 0xFFFFFFFF) as u32
    }
    pub (crate) fn update_index(&mut self, index: u32) {
        let mask = self.value.get() & 0xFFFF_FFFF_0000_0000u64;
        self.value = NonZeroU64::new((index as u64) | mask).unwrap();
    }
}
impl Default for Handle {
    fn default() -> Self {
        Handle::new(Self::INVALID_INDEX)
    }
}

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




