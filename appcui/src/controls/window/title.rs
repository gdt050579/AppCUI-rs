use crate::graphics::{CharAttribute, Character, Surface};

pub(crate) struct Title {
    text: String,
    count: u16,
    left: i32,
    right: i32,
}

impl Title {
    pub(super) fn new(text: &str) -> Self {
        let mut t = Title {
            text: String::new(),
            count: 0,
            left: 0,
            right: 0,
        };
        t.set_text(text);
        t
    }
    pub(super) fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
        self.count = text.chars().count() as u16;
    }
    pub(super) fn set_margin(&mut self, left: i32, right: i32) {
        if left + 2 >= right {
            self.left = 0;
            self.right = 0;
        } else {
            let width = (((right - (left + 2)) as u32).min(self.count as u32)) as i32;
            self.left = (left + right - width) / 2 - 1;
            self.right = self.left + width + 1;
        }
    }
    #[inline(always)]
    pub(super) fn get_text(&self) -> &str {
        self.text.as_str()
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute) {
        if self.left >= self.right {
            return;
        }
        let space = Character::with_attributes(' ', attr);
        surface.write_string(self.left + 1, 0, &self.text, attr, false);
        surface.write_char(self.left, 0, space);
        surface.write_char(self.right, 0, space);
    }
}
