use super::super::Size;

pub struct Glyph {
    size: Size,
    pub(in super::super) chars: Vec<char>,
}

impl Glyph {
    const TRANSPARENT_CHAR: char = 0 as char;
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
    pub fn with_str(width: u32, height: u32, text: &str) -> Self {
        let mut g = Glyph::new(width, height);
        g.write_str(0, 0, text);
        g
    }
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
    pub fn size(&self) -> Size {
        self.size
    }
    #[inline(always)]
    pub fn set_char(&mut self, x: u32, y: u32, ch: char) {
        if (x < self.size.width) && (y < self.size.height) {
            let pos = (y * self.size.width + x) as usize;
            self.chars[pos] = ch;
        }
    }
    #[inline(always)]
    pub fn clear_char(&mut self, x: u32, y: u32) {
        self.set_char(x, y, Glyph::TRANSPARENT_CHAR);
    }
    #[inline(always)]
    pub fn clear(&mut self) {
        self.chars.fill(Glyph::TRANSPARENT_CHAR);
    }
    #[inline(always)]
    pub fn clear_with(&mut self, ch: char) {
        self.chars.fill(ch);
    }
    #[inline(always)]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.resize_with(width, height, Glyph::TRANSPARENT_CHAR);
    }
    pub fn resize_with(&mut self, width: u32, height: u32, ch: char) {
        let sz = Glyph::compute_size(width, height);
        let new_len = (sz.width * sz.height) as usize;
        self.chars.resize(new_len, ch);
        self.clear_with(ch);
        self.size = sz;
    }
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
