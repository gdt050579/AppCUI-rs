use super::super::Size;

/// An error that can occur when parsing a string format.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StringFormatError {
    /// The string format contains multiple widths
    /// For example: `|...| |....|` contains one line with 3 pixels and another line with 4 pixels.
    MultipleWidths,
    /// The string format contains a zero height.
    ZeroHeight,
    /// The string format contains a zero width.
    ZeroWidth,
    /// The string format is too large for what underline object supports. For example a a BitTileU16 only supports a maximum of 16 pixels so a line with 17 pixels will cause this error.
    ImageTooLarge,
    /// The string format does not fit in the allocated space. For example a BitTileU16 only supports a maximum of 16 pixels so a a string representation of a 5x4 image will cause this error.
    ImageDoesNotFitInAllocatedSpace,
    /// The string format is missing a corresponding marker.
    /// For example: `|...` is missing the corresponding `|` at the end.
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
