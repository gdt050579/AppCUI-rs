use crate::graphics::{CharAttribute, Character, Surface};

enum TitleDrawMode {
    None,
    Full,
    SplitInMiddle,
}

pub(crate) struct Title {
    text: String,
    count: u16,
    left: i32,
    right: i32,
    start_part_end: usize,
    start_part_size: i32,
    end_part_start: usize,
    draw_mode: TitleDrawMode,
}

impl Title {
    pub(super) fn new(text: &str) -> Self {
        let mut t = Title {
            text: String::new(),
            count: 0,
            left: 0,
            right: 0,
            start_part_end: 0,
            end_part_start: 0,
            start_part_size: 0,
            draw_mode: TitleDrawMode::None,
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
            self.draw_mode = TitleDrawMode::None;
        } else {
            let width = (right - (left + 2)).min(self.count as i32);
            if width == (self.count as i32) {
                self.left = (left + right - width) / 2 - 1;
                self.right = self.left + width + 1;
                self.draw_mode = TitleDrawMode::Full;
            } else {
                // we should split the string in the middle
                // first part ... ending part
                // this means that the size should be at least 5 characters (start,end, ...)
                if width < 5 {
                    self.draw_mode = TitleDrawMode::None;
                } else {
                    self.start_part_size = (width - 3) / 2;
                    let first_part_end = self.start_part_size as usize;
                    let second_part_start = (self.count as usize) - (width as usize - (first_part_end + 3));
                    let mut char_index = 0usize;
                    self.start_part_end = 0;
                    self.end_part_start = 0;
                    for (offset, _) in self.text.char_indices() {
                        if char_index == first_part_end {
                            self.start_part_end = offset;
                        }
                        if char_index == second_part_start {
                            self.end_part_start = offset;
                        }
                        char_index += 1;
                    }
                    if (self.start_part_end > 0) && (self.end_part_start > self.start_part_end) {
                        self.left = (left + right - width) / 2 - 1;
                        self.right = self.left + width + 1;
                        self.draw_mode = TitleDrawMode::SplitInMiddle;
                    } else {
                        // some error
                        self.draw_mode = TitleDrawMode::None;
                    }
                }
            }
        }
    }
    #[inline(always)]
    pub(super) fn get_text(&self) -> &str {
        self.text.as_str()
    }
    #[inline(always)]
    pub(super) fn paint(&self, surface: &mut Surface, attr: CharAttribute) {
        match self.draw_mode {
            TitleDrawMode::None => {
                return;
            }
            TitleDrawMode::Full => {
                surface.write_string(self.left + 1, 0, &self.text, attr, false);
            }
            TitleDrawMode::SplitInMiddle => {
                surface.write_string(
                    self.left + 1,
                    0,
                    &self.text[..self.start_part_end],
                    attr,
                    false,
                );
                surface.write_string(self.left + 1 + self.start_part_size, 0, "...", attr, false);
                surface.write_string(
                    self.left + 4 + self.start_part_size,
                    0,
                    &self.text[self.end_part_start..],
                    attr,
                    false,
                );
            }
        }
        let space = Character::with_attributes(' ', attr);
        surface.write_char(self.left, 0, space);
        surface.write_char(self.right, 0, space);
    }
}
