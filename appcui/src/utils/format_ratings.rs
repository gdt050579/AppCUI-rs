pub struct FormatRatings;
impl FormatRatings {
    pub fn two_chars<'a>(
        empty_char: char,
        fill_char: char,
        value: u32,
        max_value: u32,
        chars: u8,
        max_chars: u8,
        buf: &'a mut [u8],
    ) -> Option<&'a str> {
        if max_value == 0 {
            return None;
        }
        let chars = chars.min(max_chars);
        let value = value.min(max_value);
        let mut empty_char_u8: [u8; 6] = [0; 6];
        let mut fill_char_u8: [u8; 6] = [0; 6];
        let sz_empty = empty_char.encode_utf8(&mut empty_char_u8).len();
        let sz_fill = fill_char.encode_utf8(&mut fill_char_u8).len();
        let min_size = sz_empty.max(sz_fill) * (max_chars as usize);
        if buf.len() < min_size {
            return None;
        }
        let pos = (value as u64) * (max_chars as u64) / (max_value as u64);
        let mut index = 0;
        let mut buf_pos = 0;
        let p = buf.as_mut_ptr();
        while index < pos {
            unsafe {
                std::ptr::copy_nonoverlapping(fill_char_u8.as_ptr(), p.add(buf_pos), sz_fill);
            }
            index += 1;
            buf_pos += sz_fill;
        }
        while index < (max_chars as u64) {
            unsafe {
                std::ptr::copy_nonoverlapping(empty_char_u8.as_ptr(), p.add(buf_pos), sz_empty);
            }
            index += 1;
            buf_pos += sz_empty;
        }
        Some(unsafe { std::str::from_utf8_unchecked(&buf[..buf_pos as usize]) })
    }
    pub fn raport<'a>(value: u32, max_value: u32, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 3 {
            return None;
        }
        let mut pos = buf.len() - 1;
        let mut v = max_value;
        loop {
            buf[pos] = (v % 10 + 48) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
            pos -= 1;
            if pos == 0 {
                return None;
            }
        }
        if pos<2 {
            return None;
        }
        pos -= 1;
        buf[pos] = b'/';
        pos -= 1;
        let mut v = value.min(max_value);
        loop {
            buf[pos] = (v % 10 + 48) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
            if pos == 0 {
                return None;
            }
            pos -= 1;
        }
        Some(unsafe { std::str::from_utf8_unchecked(&buf[pos..]) })
    }
}
