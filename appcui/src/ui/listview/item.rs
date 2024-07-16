use super::ListItem;
use crate::graphics::CharAttribute;

pub struct Item<T>
where
    T: ListItem,
{
    data: T,
    checked: bool,
    attr: Option<CharAttribute>,
    group_id: u16,
    x_ofs: i8,
    icon: [char;2],
}

impl<T> Item<T> where T: ListItem {
    pub fn new(data: T, checked: bool, attr: Option<CharAttribute>, x_offset: i8, icon_chars: [char;2]) -> Self {
        Self {
            data,
            checked,
            attr,
            group_id: 0,
            x_ofs: x_offset,
            icon: icon_chars,
        }
    }
    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.data
    }
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.data
    }
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    #[inline(always)]
    pub(super) fn x_offset(&self) -> i32 {
        self.x_ofs as i32
    }
    #[inline(always)]
    pub(super) fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }
}
impl<T> From<T> for Item<T> where T: ListItem {
    fn from(value: T) -> Self {
        Self {
            data: value,
            checked: false,
            attr: None,
            group_id: 0,
            x_ofs: 0,
            icon: [0 as char, 0 as char],
        }
    }
}