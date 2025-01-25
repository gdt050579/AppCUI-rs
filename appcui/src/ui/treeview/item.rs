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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum FoldStatus {
    Collapsed,
    Expanded,
    NonExpandable,
}

pub struct Item<T>
where
    T: ListItem,
{
    data: T,
    selected: bool,
    attr: Option<CharAttribute>,
    icon: [char; 2],
    pub(super) fold_status: FoldStatus,
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
    pub fn new(data: T, selected: bool, attr: Option<CharAttribute>, icon_chars: [char; 2]) -> Self {
        Self {
            data,
            selected,
            attr,
            depth: 0,
            line_mask: 0,
            fold_status: FoldStatus::Expanded,
            visibility: ItemVisibility::Visible,
            icon: icon_chars,
            handle: Handle::None,
            parent: Handle::None,
            children: Vec::new(),
        }
    }
    pub fn expandable(data: T, collapsed: bool) -> Self {
        let mut i = Self::from(data);
        if collapsed {
            i.fold_status = FoldStatus::Collapsed;
        } else {
            i.fold_status = FoldStatus::Expanded;
        }
        i
    }

    pub fn non_expandable(data: T) -> Self {
        let mut i = Self::from(data);
        i.fold_status = FoldStatus::NonExpandable;
        i
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
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    #[inline(always)]
    pub(super) fn set_selected(&mut self, value: bool) {
        self.selected = value;
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

    pub(super) fn x_offset(&self, fold_sign_width: u8, icon_width: u8, from_fold_button: bool) -> i32 {
        if self.depth == 0 {
            0
        } else {
            (self.depth as i32) * (3 + fold_sign_width as i32 + icon_width as i32) - if from_fold_button { 0 } else { 2 }
        }
    }

    #[inline(always)]
    pub(super) fn reverse_fold(&mut self) -> bool {
        match self.fold_status {
            FoldStatus::Collapsed => {
                self.fold_status = FoldStatus::Expanded;
                true
            }
            FoldStatus::Expanded => {
                self.fold_status = FoldStatus::Collapsed;
                true
            }
            FoldStatus::NonExpandable => false,
        }
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
            let value = previous_value & (bit - 1);
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
            selected: false,
            attr: None,
            depth: 0,
            line_mask: 0,
            fold_status: FoldStatus::Expanded,
            visibility: ItemVisibility::Visible,
            icon: [0u8 as char, 0u8 as char],
            handle: Handle::None,
            parent: Handle::None,
            children: Vec::new(),
        }
    }
}
