pub(crate) struct Glyphs {
    text: String,
    count: usize,
}
impl Glyphs {
    #[inline(always)]
    pub fn count(&self)->usize {
        self.count
    }
    #[inline(always)]
    pub fn len(&self)->usize {
        self.text.len()
    }
    #[inline(always)]
    pub fn text(&self)->&str {
        self.text.as_str()
    }

    #[inline(always)]
    fn is_variation_selectior(c: char) -> bool {
        match c {
            '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}' => true,
            _ => false,
        }
    }
    #[inline(always)]
    fn count_glyphs(text: &str) -> usize {
        text.chars().filter(|&c| !Glyphs::is_variation_selectior(c)).count()
    }

    fn character(&self, offset: usize) -> Option<char> {
        if offset >= self.text.len() || !self.text.is_char_boundary(offset) {
            return None;
        }
        self.text[offset..].chars().next()
    }
}


impl From<&str> for Glyphs {
    fn from(value: &str) -> Self {
        let mut obj = Self {
            text: String::from(value),
            count: 0
        };
        obj.count = Glyphs::count_glyphs(obj.text.as_str());
        obj
    }
}