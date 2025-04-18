use crate::graphics::*;

pub(crate) struct Symbol {
    chars: [char; 3],
}
impl Symbol {
    pub(crate) fn new(text: &str) -> Self {
        let mut chars = [0 as char; 3];
        let mut i = 0;
        for c in text.chars() {
            if i < 3 {
                chars[i] = c;
                i += 1;
            } else {
                break;
            }
        }
        Self { chars }
    }
    pub(crate) fn paint(&self, surface: &mut Surface, x: i32, y: i32, attr1: CharAttribute, attr2: CharAttribute) {
        let mut x = x;
        surface.write_char(x, y, Character::with_attributes(self.chars[0], attr1));
        if self.chars[1] != 0 as char {
            x += 1;
            surface.write_char(x, y, Character::with_attributes(self.chars[1], attr2));
            if self.chars[2] != 0 as char {
                x += 1;
                surface.write_char(x, y, Character::with_attributes(self.chars[2], attr1));
            }
        }
    }
}
