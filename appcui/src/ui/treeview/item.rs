use super::ListItem;
use crate::graphics::CharAttribute;
use crate::prelude::ColumnsHeader;
use crate::system::Handle;
use crate::utils::glyphs::GlyphParser;

#[derive(Debug, Clone, Copy)]
pub(super) enum ItemVisibility {
    Visible,
    Hidden,
    VisibleBecauseOfChildren,
}

pub struct Item<T>
where
    T: ListItem,
{
    data: T,
    checked: bool,
    attr: Option<CharAttribute>,
    icon: [char; 2],
    pub(super) visibility: ItemVisibility,
    pub(super) line_mask: u32,
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
            line_mask: 0,
            visibility: ItemVisibility::Visible,
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
    pub(super) fn is_visible(&self) -> bool {
        matches!(self.visibility, ItemVisibility::Visible | ItemVisibility::VisibleBecauseOfChildren)
    }

    #[inline(always)]
    pub(super) fn is_visible_because_of_children(&self) -> bool {
        matches!(self.visibility, ItemVisibility::VisibleBecauseOfChildren)
    }

    #[inline(always)]
    pub(super) fn has_matched(&self) -> bool {
        matches!(self.visibility, ItemVisibility::Visible)
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

    #[inline(always)]
    pub(super) fn set_line_mask(&mut self, previous_value: u32, depth: u16, last_sibling: bool) -> u32 {
        self.depth = depth;
        if (depth == 0) || (depth > 32) {
            self.line_mask = 0;
        } else {
            let bit = 1u32 << (depth - 1);
            let value = previous_value &  (bit-1);
            if !last_sibling {
                self.line_mask = value | bit;
            } else {
                self.line_mask = value;
            }
        }
        self.line_mask
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
            line_mask: 0,
            visibility: ItemVisibility::Visible,
            icon: [0u8 as char, 0u8 as char],
            handle: Handle::None,
            parent: Handle::None,
            children: Vec::new(),
        }
    }
}
