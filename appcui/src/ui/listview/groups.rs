#[derive(Copy,Clone)]
pub struct Group {
    index: u16
}
impl Group {
    #[allow(non_upper_case_globals)]
    pub const None: Group = Group { index: 0 };
    pub(super) fn new(index: u16) -> Group {
        Group { index }
    }
    #[inline(always)]
    pub(super) fn index(&self) -> u16 {
        self.index
    }
}

#[derive(Default)]
pub(crate) struct GroupInformation {
    name: String,
    name_chars_count: u16,
    items_count: u32,
    items_check_count: u32,
    collapsed: bool,
}

impl GroupInformation {
    pub(super) fn new(name: &str) -> GroupInformation {
        GroupInformation {
            name: String::from(name),
            name_chars_count: name.chars().count() as u16,
            items_count: 0,
            items_check_count: 0,
            collapsed: false,
        }
    }
    #[inline(always)]   
    pub(super) fn is_empty(&self) -> bool {
        self.items_count == 0 
    }
    #[inline(always)]
    pub(super) fn name(&self) -> &str {
        &self.name
    }
    #[inline(always)]
    pub(super) fn name_chars_count(&self) -> u16 {
        self.name_chars_count
    }
    #[inline(always)]
    pub(super) fn items_count(&self) -> u32 {
        self.items_count
    }
    #[inline(always)]
    pub(super) fn set_items_count(&mut self, value: u32) {
        self.items_count = value;
    }   
    #[inline(always)]
    pub(super) fn increment_items_count(&mut self) {
        self.items_count += 1;
    }
    #[inline(always)]
    pub(super) fn items_checked_count(&self) -> u32 {
        self.items_check_count
    }
    #[inline(always)]
    pub(super) fn set_items_checked_count(&mut self, value: u32) {
        self.items_check_count = value;
    }  
    #[inline(always)]
    pub(super) fn is_collapsed(&self) -> bool {
        self.collapsed
    }
    #[inline(always)]
    pub(super) fn set_collapsed(&mut self, value: bool) {
        self.collapsed = value;
    }
}