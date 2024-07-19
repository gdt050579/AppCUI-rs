use super::ListItem;
use super::Group;
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
    pub fn new(data: T, checked: bool, attr: Option<CharAttribute>, x_offset: i8, icon_chars: [char;2], group: Group) -> Self {
        Self {
            data,
            checked,
            attr,
            group_id: group.index(),
            x_ofs: x_offset,
            icon: icon_chars,
        }
    }
    pub fn with_group(data: T, group: Group) -> Self {
        Self {
            data,
            checked: false,
            attr: None,
            group_id: group.index(),
            x_ofs: 0,
            icon: [0 as char, 0 as char],
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
    #[inline(always)]
    pub(super) fn icon_first_character(&self) -> char {
        self.icon[0]
    }
    #[inline(always)]
    pub(super) fn icon_second_character(&self) -> char {
        self.icon[1]
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