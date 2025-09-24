use super::super::Size;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StringFormatError {
    MultipleWidths,
    ZeroHeight,
    ZeroWidth,
    ImageTooLarge,
    ImageDoesNotFitInAllocatedSpace,
    MissingCorespondingMarker,
}

pub(super) struct StringFormatParser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> StringFormatParser<'a> {
    pub(super) fn new(text: &'a str) -> Self {
        Self {
            buf: text.as_bytes(),
            pos: 0,
        }
    }
    pub(super) fn size(&self) -> Result<Size, StringFormatError> {
        let mut w = 0u32;
        let mut h = 0u32;
        let mut temp_w = 0u32;
        let mut add_value = 0u32;
        for b in self.buf {
            if (*b) == b'|' {
                add_value = 1 - add_value;
                if add_value == 0 {
                    h += 1;
                    if w == 0 {
                        w = temp_w;
                    } else if temp_w != w {
                        return Err(StringFormatError::MultipleWidths);
                    }
                    temp_w = 0;
                }
            } else {
                temp_w += add_value;
            }
        }
        if add_value==1 {
            return Err(StringFormatError::MissingCorespondingMarker);
        }
        if h == 0 {
            return Err(StringFormatError::ZeroHeight);
        }
        if w == 0 {
            return Err(StringFormatError::ZeroWidth);
        }
        Ok(Size::new(w, h))
    }
    pub(super) fn next_line(&mut self) -> Option<&'a [u8]> {
        let len = self.buf.len();
        while (self.pos < len) && (self.buf[self.pos] != b'|') {
            self.pos += 1;
        }
        if self.pos >= len {
            return None;
        }
        self.pos += 1;
        let start = self.pos;
        while (self.pos < len) && (self.buf[self.pos] != b'|') {
            self.pos += 1;
        }
        if self.pos >= len {
            return None;
        }
        let end = self.pos;
        self.pos += 1;
        Some(&self.buf[start..end])
    }
}
