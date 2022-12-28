/*
General format can be one of:
1. <word> <equal> <value> <separator> ...
2. <word> <separator>
Where:
- <equal> can be '=' or ':'
- <separator> can be ',' or ';'
- <word> and <value> = any sequance of chars different than space, tab
*/
#[repr(u8)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub(crate) enum ValueType {
    None,
    String,
    Number,
    Percentage,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CharType {
    None = 0,
    Word = 1,
    Space = 2,
    Separator = 3,
    Eq = 4,
}
static LOWER_CASE_TABLE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

fn compute_hash(buf: &[u8]) -> u64 {
    // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
    if buf.len() == 0 {
        return 0;
    }
    let mut hash = 0xcbf29ce484222325u64;
    let mut idx = 0usize;
    while idx < buf.len() {
        hash = hash ^ (LOWER_CASE_TABLE[buf[idx] as usize] as u64);
        //hash = hash * 0x00000100000001B3u64;
        hash = hash.wrapping_mul(0x00000100000001B3u64);
        idx += 1;
    }
    return hash;

}
#[derive(Debug,PartialEq)]
pub(crate) struct KeyValuePair<'a> {
    pub(crate) key_hash: u64,
    pub(crate) key: &'a str,
    pub(crate) value_hash: u64,
    pub(crate) value: &'a str,
    pub(crate) numerical_value: i32,
    pub(crate) value_type: ValueType,
}
pub(crate) struct KeyValueParser<'a> {
    text: &'a str,
    text_buffer: &'a [u8],
    current: usize,
    end: usize,
    item: KeyValuePair<'a>,
}

impl<'a> KeyValueParser<'a> {
    pub fn new(text_representation: &str) -> KeyValueParser {
        KeyValueParser {
            text: text_representation,
            text_buffer: text_representation.as_bytes(),
            current: 0,
            end: text_representation.len(),
            item: KeyValuePair {
                key_hash: 0,
                key: "",
                value_hash: 0,
                value: "",
                numerical_value: 0,
                value_type: ValueType::None,
            },
        }
    }
    #[inline]
    fn get_current_char_type(&self) -> CharType {
        if self.current >= self.end {
            return CharType::None;
        }
        let ch = self.text_buffer[self.current];
        match ch {
            b' ' | b'\t' => {
                return CharType::Space;
            }
            b';' | b',' => {
                return CharType::Separator;
            }
            b':' | b'=' => {
                return CharType::Eq;
            }
            _ => {
                return CharType::Word;
            }
        }
    }
    #[inline]
    fn skip(&mut self, char_type: CharType) {
        while self.get_current_char_type() == char_type {
            self.current += 1;
        }
    }
    #[inline]
    fn parse_word(&mut self) -> usize {
        loop {
            self.skip(CharType::Word);
            if self.get_current_char_type() != CharType::Space {
                return self.current;
            }
            let cpos = self.current;
            self.skip(CharType::Space);
            if self.get_current_char_type() != CharType::Word {
                self.current = cpos;
                return cpos;
            }
        }
    }
    #[inline]
    fn analize_value(&mut self, buf: &[u8]) {
        let mut negative = false;
        let mut is_percentage = false;
        let mut first_part = 0i32;
        let mut second_part = 0i32;
        let mut pos = 0usize;
        let end = buf.len();

        if (end > 0) && (buf[0] == b'-') {
            negative = true;
            pos += 1;
        }
        while (pos < end) && ((buf[pos] >= b'0') && (buf[pos] <= b'9')) {
            first_part = first_part * 10 + ((buf[pos] - b'0') as i32);
            pos += 1;
        }
        if (pos < end) && (buf[pos] == b'.') {
            let mut cnt = 0;
            while (pos < end) && ((buf[pos] >= b'0') && (buf[pos] <= b'9')) {
                if cnt < 2 {
                    second_part = second_part * 10 + ((buf[pos] - b'0') as i32);
                    cnt += 1;
                }
                pos += 1;
            }
        }
        if (pos < end) && (buf[pos] == b'%') {
            is_percentage = true;
            pos += 1;
        }
        if pos < end {
            self.item.value_type = ValueType::String;
            return;
        }
        // valid number
        if is_percentage {
            self.item.numerical_value = first_part * 100 + (second_part % 100);
            if negative {
                self.item.numerical_value = -self.item.numerical_value;
            }
            self.item.value_type = ValueType::Percentage;
        } else {
            self.item.numerical_value = first_part;
            if negative {
                self.item.numerical_value = -self.item.numerical_value;
            }
            self.item.value_type = ValueType::Number;
        }
    }
    pub fn next(&mut self) -> Option<&KeyValuePair> {
        self.skip(CharType::Space);
        if self.current >= self.end {
            return None;
        }
        if self.get_current_char_type() != CharType::Word {
            return None;
        }
        let key_start = self.current;
        let key_end = self.parse_word();
        self.skip(CharType::Space);
        self.item.key_hash = compute_hash(&self.text_buffer[key_start..key_end]);
        self.item.key = &self.text[key_start..key_end];
        self.item.numerical_value = 0;

        if self.get_current_char_type() == CharType::Eq {
            self.current += 1;
            self.skip(CharType::Space);
            if self.get_current_char_type() != CharType::Word {
                return None;
            }
            let value_start = self.current;
            let value_end = self.parse_word();
            self.skip(CharType::Space);
            self.item.value = &self.text[value_start..value_end];
            self.item.value_hash = compute_hash(&self.text_buffer[value_start..value_end]);
            self.analize_value(&self.text_buffer[value_start..value_end]);
        } else {
            // empty
            self.item.value_type = ValueType::None;
        }
        self.skip(CharType::Space);
        if self.get_current_char_type() == CharType::Separator {
            self.current += 1;
        }
        return Some(&self.item);
    }
}
