#[derive(Copy,Clone)]
pub struct Group {
    index: u16
}
impl Group {
    pub const None: Group = Group { index: 0 };
    pub(super) fn new(index: u16) -> Group {
        Group { index }
    }
    #[inline(always)]
    pub(super) fn index(&self) -> u16 {
        self.index
    }
}

pub(crate) struct GroupInformation {
    pub(super) name: String,
    pub(super) items_count: u32,
}

impl GroupInformation {
    #[inline(always)]   
    pub(super) fn is_empty(&self) -> bool {
        self.items_count == 0
    }
}

impl Default for GroupInformation {
    fn default() -> Self {
        Self {
            name: String::new(),
            items_count: 0,
        }
    }
}