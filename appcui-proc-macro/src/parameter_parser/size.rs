#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Size {
    pub(crate) width: u16,
    pub(crate) height: u16,
}
impl Size {
    pub(super) fn from_str(text: &str) -> Option<Size> {
        let mut width = 0u64;
        let mut height = 0u64;
        let mut index = 0;
        let buf = text.as_bytes();
        // skip spaces
        while (index < buf.len()) && (buf[index] == b' ') {
            index += 1;
        }
        if index >= buf.len() {
            return None;
        }
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return None;
        }
        while (index < buf.len()) && (buf[index] >= b'0') && (buf[index] <= b'9') {
            width = width * 10 + (buf[index] - b'0') as u64;
            index += 1;
        }
        // skip spaces
        while (index < buf.len()) && (buf[index] == b' ') {
            index += 1;
        }
        if index >= buf.len() {
            return None;
        }
        // expect either 'x' or ','
        if (buf[index] != b',') && (buf[index] != b'x') && (buf[index] != b'X') {
            return None;
        }
        index += 1;
        while (index < buf.len()) && (buf[index] == b' ') {
            index += 1;
        }
        if index >= buf.len() {
            return None;
        }
        if (buf[index] < b'0') || (buf[index] > b'9') {
            return None;
        }
        while (index < buf.len()) && (buf[index] >= b'0') && (buf[index] <= b'9') {
            height = height * 10 + (buf[index] - b'0') as u64;
            index += 1;
        }
        while (index < buf.len()) && (buf[index] == b' ') {
            index += 1;
        }
        // we should be at the end of the string
        if index != buf.len() {
            return None;
        }
        if (width == 0) || (width > 32000) || (height == 0) || (height > 32000) {
            return None;
        }
        Some(Size {
            width: width as u16,
            height: height as u16,
        })
    }
}
