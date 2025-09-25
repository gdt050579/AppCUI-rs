use super::super::Size;

pub struct Glyph {
    size: Size,
    pub(in super::super) chars: Vec<char>,
}

impl Glyph {
    const TRANSPARENT_CHAR: char = 0 as char;
    /// Creates a new glyph with the specified width and height.
    /// The glyph is initialized with the transparent character.
    /// if the size of the glyph is bigger than 1024x1024, it will be clamped to 1024x1024.
    pub fn new(width: u32, height: u32) -> Self {
        let sz = Glyph::compute_size(width, height);

        if sz.width == 0 {
            Self { size: sz, chars: Vec::new() }
        } else {
            let new_len = sz.width as usize * sz.height as usize;
            let mut v = Vec::with_capacity(new_len);
            v.resize(new_len, Glyph::TRANSPARENT_CHAR);
            Self { size: sz, chars: v }
        }
    }
    /// Creates a new glyph with the specified width and height and initializes it with the specified text.
    /// The text is written from the top left corner of the glyph.
    pub fn with_str(width: u32, height: u32, text: &str) -> Self {
        let mut g = Glyph::new(width, height);
        g.write_str(0, 0, text);
        g
    }

    /// Returns the character at the specified position or None if the position is outside the glyph.
    #[inline(always)]
    pub fn char(&self, x: u32, y: u32) -> Option<char> {
        if (x < self.size.width) && (y < self.size.height) {
            let pos = (y * self.size.width + x) as usize;
            Some(self.chars[pos])
        } else {
            None
        }
    }
    #[inline(always)]
    /// Returns the size of the glyph.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Sets the character at the specified position. If the position is outside the glyph, the character will not be set.
    #[inline(always)]
    pub fn set_char(&mut self, x: u32, y: u32, ch: char) {
        if (x < self.size.width) && (y < self.size.height) {
            let pos = (y * self.size.width + x) as usize;
            self.chars[pos] = ch;
        }
    }
    #[inline(always)]
    /// Clears the character at the specified position.
    pub fn clear_char(&mut self, x: u32, y: u32) {
        self.set_char(x, y, Glyph::TRANSPARENT_CHAR);
    }
    #[inline(always)]
    /// Clears the entire glyph.
    pub fn clear(&mut self) {
        self.chars.fill(Glyph::TRANSPARENT_CHAR);
    }
    #[inline(always)]
    /// Fills the entire glyph with the specified character.
    pub fn fill(&mut self, ch: char) {
        self.chars.fill(ch);
    }
    #[inline(always)]
    /// Resizes the glyph to the specified width and height.
    /// The glyph is cleared with the transparent character.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize_with(width, height, Glyph::TRANSPARENT_CHAR);
    }

    /// Resizes the glyph to the specified width and height and fills it with the specified character.
    pub fn resize_with(&mut self, width: u32, height: u32, ch: char) {
        let sz = Glyph::compute_size(width, height);
        let new_len = (sz.width * sz.height) as usize;
        self.chars.resize(new_len, ch);
        self.fill(ch);
        self.size = sz;
    }
    /// Writes the specified text to the glyph starting from the specified position.
    /// The text is written from the top left corner of the glyph.
    /// If the text is outside the glyph, it will not be written.
    /// The text can contain multiple line.
    ///
    /// # Example
    ///
    /// ```rust
    /// use appcui::prelude::*;
    ///
    /// let mut glyph = image::Glyph::new(10, 10);
    /// glyph.write_str(0, 0, "Hello, world!");
    /// ```
    /// 
    pub fn write_str(&mut self, x: u32, y: u32, text: &str) {
        if (x >= self.size.width) || (y >= self.size.height) {
            return;
        }
        let mut start_offset = (y * self.size.width + x) as usize;
        let mut ofs = start_offset;
        let mut x_coord = x;
        let mut y_coord = y;
        for ch in text.chars() {
            if ch == '\n' {
                y_coord += 1;
                if y_coord >= self.size.height {
                    break;
                }
                start_offset += self.size.width as usize;
                ofs = start_offset;
                x_coord = x;
                continue;
            }
            if x_coord < self.size.width {
                self.chars[ofs] = ch;
                x_coord += 1;
                ofs += 1;
            }
        }
    }
    #[inline(always)]
    fn compute_size(width: u32, height: u32) -> Size {
        if width * height == 0 {
            Size::new(0, 0)
        } else {
            Size::new(width.min(1024), height.min(1024))
        }
    }
}
