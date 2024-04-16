pub(crate) struct Glyphs {
    text: String,
    count: usize,
}
impl Glyphs {
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.text.len()
    }
    #[inline(always)]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    #[inline(always)]
    fn is_variation_selector(c: char) -> bool {
        matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
    }
    #[inline(always)]
    fn count_glyphs(text: &str) -> usize {
        text.chars().filter(|&c| !Glyphs::is_variation_selector(c)).count()
    }

    pub fn character(&self, offset: usize) -> Option<(char, u32)> {
        let mut chars = self.text[offset..].chars();
        chars.next().map(|first_char| {
            let mut char_size = first_char.len_utf8() as u32;
            if char_size > 1 {
                if let Some(next_char) = chars.next() {
                    if Glyphs::is_variation_selector(next_char) {
                        char_size += next_char.len_utf8() as u32;
                    }
                }
            }
            (first_char, char_size)
        })
    }
    pub fn prev_character(&self, offset: usize) -> Option<(char, u32)> {
        if offset == 0 || offset > self.text.len() {
            return None;
        }
        let slice = &self.text[..offset];
        let mut char_indices = slice.char_indices().rev();
        let mut total_size = 0;

        if let Some((_, previous_char)) = char_indices.next() {
            total_size += previous_char.len_utf8() as u32;
            if Glyphs::is_variation_selector(previous_char) {
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

}

impl From<&str> for Glyphs {
    fn from(value: &str) -> Self {
        let mut obj = Self {
            text: String::from(value),
            count: 0,
        };
        obj.count = Glyphs::count_glyphs(obj.text.as_str());
        obj
    }
}
