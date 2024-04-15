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
    fn is_variation_selectior(c: char) -> bool {
        matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
    }
    #[inline(always)]
    fn count_glyphs(text: &str) -> usize {
        text.chars().filter(|&c| !Glyphs::is_variation_selectior(c)).count()
    }

    pub fn character(&self, offset: usize) -> Option<(char, u32)> {
        let mut chars = (&self.text[offset..]).chars();
        chars.next().map(|first_char| {
            let mut char_size = first_char.len_utf8() as u32;
            if char_size > 1 {
                if let Some(next_char) = chars.next() {
                    if Glyphs::is_variation_selectior(next_char) {
                        char_size += next_char.len_utf8() as u32;
                    }
                }
            }
            (first_char, char_size)
        })
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
