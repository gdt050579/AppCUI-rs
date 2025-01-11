use super::ListItem;
use crate::graphics::CharAttribute;
use crate::prelude::ColumnsHeader;
use crate::system::Handle;
use crate::utils::glyphs::GlyphParser;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
pub(super) enum ItemFlags {
    LastSibling = 0x01,
    
}

pub struct Item<T>
where
    T: ListItem,
{
    data: T,
    checked: bool,
    attr: Option<CharAttribute>,
    icon: [char; 2],
    flags: ItemFlags,
    pub(super) depth: u16,
    pub(super) handle: Handle<Item<T>>,
    pub(super) parent: Handle<Item<T>>,
    pub(super) children: Vec<Handle<Item<T>>>,
}

impl<T> Item<T>
where
    T: ListItem,
{
    pub fn new(data: T, checked: bool, attr: Option<CharAttribute>, icon_chars: [char; 2]) -> Self {
        Self {
            data,
            checked,
            attr,
            depth: 0,
            flags: ItemFlags::None,
            icon: icon_chars,
            handle: Handle::None,
            parent: Handle::None,
            children: Vec::new(),
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
    pub(super) fn render_attr(&self) -> Option<CharAttribute> {
        self.attr
    }

    #[inline(always)]
    pub(super) fn matches(&self, search_text: &str, header: Option<&ColumnsHeader>) -> bool {
        if search_text.is_empty() {
            true
        } else {
            if let Some(header) = header {
                let mut output: [u8; 256] = [0; 256];
                let columns_count = header.columns().len();
                for column_index in 0..columns_count {
                    if let Some(rm) = self.data.render_method(column_index as u16) {
                        if let Some(item_text) = rm.string_representation(&mut output) {
                            if item_text.index_ignoring_case(search_text).is_some() {
                                return true;
                            }
                        }
                    }
                }
                false
            } else {
                self.data.matches(search_text)
            }
        }
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
            depth: 0,
            flags: ItemFlags::None,
            icon: [0u8 as char, 0u8 as char],
            handle: Handle::None,
            parent: Handle::None,
            children: Vec::new(),
        }
    }
}
