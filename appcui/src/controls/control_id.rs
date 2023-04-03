use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone, PartialEq)]
pub (crate) struct ControlID {
    id: u32,
}

static GLOBAL_VERSION: AtomicUsize = AtomicUsize::new(0);
impl ControlID {
    pub fn new()->Self {
        Self {
            id: (GLOBAL_VERSION.fetch_add(1, Ordering::SeqCst) & 0xFFFFFFFE) as u32
        }
    }
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.id != u32::MAX
    }
    pub const INVALID: Self = Self {
        id: u32::MAX
    };
}