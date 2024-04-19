pub(crate) trait GlyphParser {
    fn count_glyphs(&self) -> usize;
    fn glyph(&self, offset: usize) -> Option<(char, u32)>;
    fn previous_glyph(&self, offset: usize) -> Option<(char, u32)>;
    fn next_pos(&self, current_pos: usize, count_glyphs: usize) -> usize;
    fn previous_pos(&self, current_pos: usize, count_glyphs: usize) -> usize;
}

#[inline(always)]
fn is_variation_selector(c: char) -> bool {
    matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
}

impl GlyphParser for String {
    fn count_glyphs(&self) -> usize {
        self.chars().filter(|&c| !is_variation_selector(c)).count()
    }
    fn glyph(&self, offset: usize) -> Option<(char, u32)> {
        let mut chars = self[offset..].chars();
        chars.next().map(|first_char| {
            let mut char_size = first_char.len_utf8() as u32;
            if char_size > 1 {
                if let Some(next_char) = chars.next() {
                    if is_variation_selector(next_char) {
                        char_size += next_char.len_utf8() as u32;
                    }
                }
            }
            (first_char, char_size)
        })
    }

    fn previous_glyph(&self, offset: usize) -> Option<(char, u32)> {
        if offset == 0 || offset > self.len() {
            return None;
        }
        let slice = &self[..offset];
        let mut char_indices = slice.char_indices().rev();
        let mut total_size = 0;

        if let Some((_, previous_char)) = char_indices.next() {
            total_size += previous_char.len_utf8() as u32;
            if is_variation_selector(previous_char) {
                if let Some((_, char_before_previous_char)) = char_indices.next() {
                    total_size += char_before_previous_char.len_utf8() as u32;
                    return Some((char_before_previous_char, total_size));
                }
            }
            Some((previous_char, total_size))
        } else {
            None
        }
    }

    fn next_pos(&self, current_pos: usize, count_glyphs: usize) -> usize {
        let len = self.len();
        let mut count = count_glyphs;
        let mut pos = current_pos;
        while (count > 0) && (pos < len) {
            if let Some((_, sz)) = self.glyph(pos) {
                pos += sz as usize;
            } else {
                break;
            }
            count -= 1;
        }
        pos
    }

    fn previous_pos(&self, current_pos: usize, count_glyphs: usize) -> usize {
        let mut count = count_glyphs;
        let mut pos = current_pos;
        while (count > 0) && (pos > 0) {
            if let Some((_, sz)) = self.previous_glyph(pos) {
                let sz = sz as usize;
                if sz > pos {
                    pos = 0;
                } else {
                    pos -= sz;
                }
            } else {
                break;
            }
            count -= 1;
        }
        pos
    }
}
