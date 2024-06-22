pub struct Item {
    pub(super) value: String,
    pub(super) count: u32,
    pub(super) left_char_offset: u32,
    pub(super) left: u32,
    pub(super) checked: bool,
    pub(super) filtered: bool
}

impl Item {
    /// Creates a new item with the specified text and checked state. 
    /// 
    /// # Example
    /// ```rust, no_run
    /// use appcui::ui::listbox::Item;
    /// let item = Item::new("Item 1", false);
    /// ```
    pub fn new(text: &str, checked: bool) -> Self {
        Item {
            value: String::from(text),
            count: text.chars().count() as u32,
            left: 0,
            left_char_offset: 0,
            checked,
            filtered: true
        }
    }

    /// Returns the text of the item.
    #[inline(always)]
    pub fn text(&self) -> &str {
        &self.value
    }

    /// Returns true if the item is checked or false otherwise.
    #[inline(always)]
    pub fn is_checked(&self) -> bool {
        self.checked
    }
    #[inline(always)]
    pub(super) fn visible_text(&self) -> &str {
        if self.left_char_offset >= self.value.len() as u32 {
            ""
        } else {
            &self.value[self.left_char_offset as usize..]
        }
    }
    pub(super) fn update_left_pos(&mut self, new_left: u32) {
        if new_left != self.left {
            self.left = new_left;
            if new_left == 0 {
                self.left_char_offset = 0;
            } else if new_left > self.count {
                self.left_char_offset = u32::MAX;
            } else if let Some(offset) = self.value.char_indices().nth(new_left as usize).map(|(offset, _)| offset) {
                self.left_char_offset = offset as u32;
            } else {
                self.left_char_offset = u32::MAX;
            }
        }
    }
}
