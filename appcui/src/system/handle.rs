use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Handle {
    value: u64,
}

impl Handle {
    pub const None: Handle = Handle { value: u64::MAX };
    pub(crate) fn new(index: u32) -> Self {
        let id = ((GLOBAL_ID.fetch_add(1, Ordering::SeqCst) as u32) % 0xFFFF_FFFE) as u64;
        Self {
            value: (index as u64) | (id << 32),
        }
    }
    #[inline(always)]
    pub(crate) fn get_index(&self) -> usize {
        (self.value & 0xFFFFFFFF) as usize
    }
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.value == u64::MAX
    }
}
impl Default for Handle {
    fn default() -> Self {
        Handle{
            value: u64::MAX
        }
    }
}
