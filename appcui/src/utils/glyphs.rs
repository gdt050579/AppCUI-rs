pub(crate) trait GlyphParser {
    fn count_glyphs(&self) -> usize;
    fn glyph(&self, offset: usize) -> Option<(char, u32)>;
    fn previous_glyph(&self, offset: usize) -> Option<(char, u32)>;
    fn next_pos(&self, current_pos: usize, count_glyphs: usize) -> usize;
    fn previous_pos(&self, current_pos: usize, count_glyphs: usize) -> usize;
    fn word_range(&self, current_pos: usize, is_word_char: fn(char) -> bool) -> Option<(usize, usize)>;
    fn index_ignoring_case(&self, to_find: &str) -> Option<usize>;
}

#[inline(always)]
fn is_variation_selector(c: char) -> bool {
    matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
}
#[inline(always)]
fn to_ascii_lowercase(c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        c + 32
    } else {
        c
    }
}

impl GlyphParser for str {
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

    fn word_range(&self, current_pos: usize, is_word_char: fn(char) -> bool) -> Option<(usize, usize)> {
        if current_pos >= self.len() {
            return None;
        }
        if let Some((ch, _)) = self.glyph(current_pos) {
            if !is_word_char(ch) {
                return None;
            }
            let len = self.len();
            let mut end = current_pos;
            while end < len {
                if let Some((ch, sz)) = self.glyph(end) {
                    if is_word_char(ch) {
                        end += sz as usize;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            // we found 'end' of word
            let mut start = current_pos;
            while start > 0 {
                if let Some((ch, sz)) = self.previous_glyph(start) {
                    if is_word_char(ch) {
                        if sz as usize > start {
                            start = 0;
                            break;
                        } else {
                            start -= sz as usize;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            return Some((start, end));
        }
        None
    }
    fn index_ignoring_case(&self, to_find: &str) -> Option<usize> {
        let to_find_buf = to_find.as_bytes();
        let self_buf = self.as_bytes();
        if to_find_buf.len() > self_buf.len() {
            return None;
        }
        if to_find_buf.is_empty() {
            return Some(0);
        }
        let dif = self_buf.len() - to_find_buf.len();
        let to_find_buf_size = to_find_buf.len();
        for i in 0..=dif {
            let mut match_found = true;
            for j in 0..to_find_buf_size {
                if to_ascii_lowercase(self_buf[i + j]) != to_ascii_lowercase(to_find_buf[j]) {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                return Some(i);
            }
        }
        None
    }
}
