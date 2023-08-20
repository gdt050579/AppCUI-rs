use std::{
    marker::PhantomData,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::ui::common::UIElement;

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

pub(crate) trait HandleSupport {
    fn get_handle(&self) -> Handle<UIElement>;
    fn set_handle(&mut self, handle: Handle<UIElement>);
}

pub struct Handle<T> {
    value: u64,
    _phantom: PhantomData<T>,
}
impl<T> Handle<T> {
    #[allow(non_upper_case_globals)]
    pub const None: Handle<T> = Handle {
        value: u64::MAX,
        _phantom: PhantomData,
    };
    pub(crate) fn new(index: u32) -> Self {
        let id = ((GLOBAL_ID.fetch_add(1, Ordering::SeqCst) as u32) % 0xFFFF_FFFE) as u64;
        Self {
            value: (index as u64) | (id << 32),
            _phantom: PhantomData,
        }
    }
    #[inline(always)]
    pub(crate) fn get_index(&self) -> usize {
        (self.value & 0xFFFFFFFF) as usize
    }
    #[inline(always)]
    pub(crate) fn cast<U>(&self) -> Handle<U> {
        let r: Handle<U> = Handle {
            value: self.value,
            _phantom: PhantomData,
        };
        r
    }
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.value == u64::MAX
    }
}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantom: PhantomData,
        }
    }
}
impl<T> Copy for Handle<T> {}
impl<T, U> PartialEq<Handle<T>> for Handle<U> {
    fn eq(&self, other: &Handle<T>) -> bool {
        self.value == other.value
    }
}
impl<T> Default for Handle<T> {
    fn default() -> Self {
        Handle {
            value: u64::MAX,
            _phantom: PhantomData,
        }
    }
}
