use flat_string::FlatString;

pub enum UnicodeSymbols {
    Ascii,
    Arrows,
    Animals,
    Braille,
    Blocks,
    BoxDrawing,
    Currency,
    Emoticons,
    Shapes,
    Latin,
    Punctuation
}

struct UnicodeInterval {
    start: u32,
    end: u32,
}
impl UnicodeInterval {
    const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
    #[inline(always)]
    fn size(&self) -> u32 {
        self.end + 1 - self.start
    }
}

static ANIMALS: &'static [UnicodeInterval] = &[UnicodeInterval::new(0x1F400, 0x1F43C), UnicodeInterval::new(0x1F980, 0x1F9AE)];
static ARROWS: &'static [UnicodeInterval] = &[
    UnicodeInterval::new(0x2190, 0x21FF),
    UnicodeInterval::new(0x2798, 0x27AF),
    UnicodeInterval::new(0x27B1, 0x27BE),
    UnicodeInterval::new(0x27F0, 0x27FF),
    UnicodeInterval::new(0x2B00, 0x2B11),
    UnicodeInterval::new(0x2B60, 0x2BB8),
    UnicodeInterval::new(0x1F800, 0x1F8B1),
];
static SHAPES: &'static [UnicodeInterval] = &[UnicodeInterval::new(0x25A0, 0x25FF), UnicodeInterval::new(0x1F780, 0x1F7FF)];
static PUNCTUATION: &'static [UnicodeInterval] = &[
    UnicodeInterval::new(0x2010, 0x2027),
    UnicodeInterval::new(0x2030, 0x205E),
    UnicodeInterval::new(0x2E00, 0x2E52),
    UnicodeInterval::new(0x3001, 0x3020),
];
static LATIN: &'static [UnicodeInterval] = &[
    UnicodeInterval::new(0x20, 0x7E),
    UnicodeInterval::new(0x100, 0x17F),
    UnicodeInterval::new(0xC0, 0xFF),
    UnicodeInterval::new(0x1E00, 0x1EFF),
];
enum SetData {
    Interval(u32),
    List(Vec<char>),
    MultiIntervals(&'static [UnicodeInterval]),
}
pub struct Set {
    name: FlatString<22>,
    count: u32,
    data: SetData,
}
impl Set {
    pub fn from_unicode_symbols(name: &str, symbols: UnicodeSymbols) -> Self {
        match symbols {
            UnicodeSymbols::Ascii => Self::with_interval(name, 0x20, 0x7E).unwrap(),
            UnicodeSymbols::Braille => Self::with_interval(name, 0x2800, 0x28FF).unwrap(),
            UnicodeSymbols::Blocks => Self::with_interval(name, 0x2580, 0x259F).unwrap(),
            UnicodeSymbols::BoxDrawing => Self::with_interval(name, 0x2500, 0x257F).unwrap(),
            UnicodeSymbols::Currency => Self::with_interval(name, 0x20A0, 0x20CF).unwrap(),
            UnicodeSymbols::Emoticons => Self::with_interval(name, 0x1F600, 0x1F64F).unwrap(),
            UnicodeSymbols::Animals => Self::with_multi_intervale(name, ANIMALS),
            UnicodeSymbols::Arrows => Self::with_multi_intervale(name, ARROWS),
            UnicodeSymbols::Shapes => Self::with_multi_intervale(name, SHAPES),
            UnicodeSymbols::Punctuation => Self::with_multi_intervale(name, PUNCTUATION),
            UnicodeSymbols::Latin => Self::with_multi_intervale(name, LATIN),
        }
    }
    fn with_multi_intervale(name: &str, mi: &'static [UnicodeInterval]) -> Self {
        let mut count = 0;
        for i in mi {
            count += i.size();
        }
        Self {
            name: FlatString::from_str(name),
            count,
            data: SetData::MultiIntervals(mi),
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
            SetData::MultiIntervals(multi_intervals) => {
                let mut pos = 0;
                for i in *multi_intervals {
                    let next = pos + i.size();
                    if (index >= pos) && (index < next) {
                        return char::from_u32(index - pos + i.start);
                    }
                    pos = next;
                }
                None
            }
        }
    }

    #[inline(always)]
    pub(super) fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline(always)]
    pub(super) fn name_chars_count(&self) -> usize {
        self.name.chars_count()
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
            SetData::MultiIntervals(multi_intervals) => {
                let code_point = ch as u32;
                let mut block_start = 0;
                for i in *multi_intervals {
                    if (code_point >= i.start) && (code_point <= i.end) {
                        return Some(block_start + code_point - i.start);
                    }
                    block_start += i.size();
                }
                None
            }
        }
    }
}
