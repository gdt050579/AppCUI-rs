use super::ListItem;
use crate::graphics::CharAttribute;

pub struct Item<T>
where
    T: ListItem,
{
    data: T,
    selected: bool,
    attr: Option<CharAttribute>,
    group_id: u16,
    x_ofs: i8,
    icon: [char;2],
}

impl<T> Item<T> where T: ListItem {
    pub fn new(data: T, selected: bool, attr: Option<CharAttribute>, x_offset: i8, icon_chars: [char;2]) -> Self {
        Self {
            data,
            selected,
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
}
impl<T> From<T> for Item<T> where T: ListItem {
    fn from(value: T) -> Self {
        Self {
            data: value,
            selected: false,
            attr: None,
            group_id: 0,
            x_ofs: 0,
            icon: [0 as char, 0 as char],
        }
    }
}