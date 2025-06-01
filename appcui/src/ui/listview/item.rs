use super::Group;
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
    icon: [char; 2],
}

impl<T> Item<T>
where
    T: ListItem,
{
    pub fn new(data: T, checked: bool, attr: Option<CharAttribute>, icon_chars: [char; 2], group: Group) -> Self {
        Self {
            data,
            checked,
            attr,
            group_id: group.index(),
            icon: icon_chars,
        }
    }
    pub fn with_group(data: T, group: Group) -> Self {
        Self {
            data,
            checked: false,
            attr: None,
            group_id: group.index(),
            icon: [0u8 as char, 0u8 as char],
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
    #[inline(always)]
    pub(super) fn group_id(&self) -> u16 {
        self.group_id
    }
    #[inline(always)]
    pub(super) fn render_attr(&self) -> Option<CharAttribute> {
        self.attr
    }
}
impl<T> From<T> for Item<T>
where
    T: ListItem,
{
    fn from(value: T) -> Self {
        Self {
            data: value,
            checked: false,
            attr: None,
            group_id: 0,
            icon: [0u8 as char, 0u8 as char],
        }
    }
}
