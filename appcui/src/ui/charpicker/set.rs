use flat_string::FlatString;

pub enum UnicodeSymbols {
    Ascii,
    Braille,
    Blocks,
    BoxDrawing,
    Currency,
    Emoticons,
}

enum SetData {
    Interval(u32),
    List(Vec<char>),
}
pub struct Set {
    name: FlatString<22>,
    count: u32,
    data: SetData,
}
impl Set {
    pub fn from_unicode_symbols(name: &str, symbols: UnicodeSymbols) -> Self {
        match symbols {
            UnicodeSymbols::Ascii => Self::with_interval(name, 32, 127).unwrap(),
            UnicodeSymbols::Braille => Self::with_interval(name, 0x2800, 0x28FF).unwrap(),
            UnicodeSymbols::Blocks => Self::with_interval(name, 0x2580, 0x259F).unwrap(),
            UnicodeSymbols::BoxDrawing => Self::with_interval(name, 0x2500, 0x257F).unwrap(),
            UnicodeSymbols::Currency => Self::with_interval(name, 0x20A0, 0x20CF).unwrap(),
            UnicodeSymbols::Emoticons => Self::with_interval(name, 0x1F600, 0x1F64F).unwrap(),
        }
    }
    pub fn with_interval(name: &str, start_code_point: u32, end_code_point: u32) -> Option<Self> {
        let start = start_code_point.min(end_code_point);
        let end = start_code_point.max(end_code_point);
        if (char::from_u32(start).is_none()) || (char::from_u32(start).is_none()) {
            // invalid characters
            return None;
        }
        Some(Self {
            name: FlatString::from_str(name),
            count: end + 1 - start,
            data: SetData::Interval(start),
        })
    }
    pub fn new(name: &str, string_list: &str) -> Option<Self> {
        if string_list.is_empty() {
            return None;
        }
        let v: Vec<char> = string_list.chars().collect();
        Some(Self {
            name: FlatString::from_str(name),
            count: v.len() as u32,
            data: SetData::List(v),
        })
    }

    #[inline(always)]
    pub(super) fn count(&self) -> u32 {
        self.count
    }

    #[inline(always)]
    pub(super) fn char(&self, index: u32) -> Option<char> {
        if index >= self.count {
            return None;
        }
        match &self.data {
            SetData::Interval(start) => char::from_u32(start + index),
            SetData::List(items) => Some(items[index as usize]),
        }
    }

    #[inline(always)]
    pub(super) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(super) fn index_of(&self, ch: char) -> Option<u32> {
        match &self.data {
            SetData::Interval(start) => {
                let code_point = ch as u32;
                let start = *start;
                if (code_point >= start) && (code_point < (start + self.count)) {
                    Some(code_point - start)
                } else {
                    None
                }
            }
            SetData::List(items) => items.iter().position(|&c| c == ch).map(|idx| idx as u32),
        }
    }
}
