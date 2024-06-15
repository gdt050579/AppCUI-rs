pub struct Item {
    pub(super) value: String,
    pub(super) count: u32,
    pub(super) left_char_offset: u32,
    pub(super) left: u32,
}

impl Item {
    pub fn new(text: &str) -> Self {
        Item {
            value: String::from(text),
            count: text.chars().count() as u32,
            left: 0,
            left_char_offset: 0,
        }
    }
    #[inline(always)]
    pub(super) fn text(&self) -> &str {
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
            } else {
                if new_left > self.count {
                    self.left_char_offset = u32::MAX;
                } else {
                    if let Some(offset) = self.value.char_indices().nth(new_left as usize).map(|(offset, _)| offset) {
                        self.left_char_offset = offset as u32;
                    } else {
                        self.left_char_offset = u32::MAX;
                    }
                }
            }
        }
    }
}
