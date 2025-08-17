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
    Punctuation,
    Cyrillic,
    Greek,
    Arabic,
    Chinese,
    Math,
    Subscripts,
    Superscripts,
    Numbers,
    Pictographs,
    Transport,
    Unicode,
    Games,
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

static ANIMALS: &[UnicodeInterval] = &[UnicodeInterval::new(0x1F400, 0x1F43C), UnicodeInterval::new(0x1F980, 0x1F9AE)];
static ARROWS: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x2190, 0x21FF),
    UnicodeInterval::new(0x2798, 0x27AF),
    UnicodeInterval::new(0x27B1, 0x27BE),
    UnicodeInterval::new(0x27F0, 0x27FF),
    UnicodeInterval::new(0x2B00, 0x2B11),
    UnicodeInterval::new(0x2B60, 0x2BB8),
    UnicodeInterval::new(0x1F800, 0x1F8B1),
];
static SHAPES: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x25A0, 0x25FF),
    UnicodeInterval::new(0x2BB9, 0x2BCF),
    UnicodeInterval::new(0x1F780, 0x1F7FF),
];
static PUNCTUATION: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x2010, 0x2027),
    UnicodeInterval::new(0x2030, 0x205E),
    UnicodeInterval::new(0x2E00, 0x2E52),
    UnicodeInterval::new(0x3001, 0x3020),
];
static LATIN: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x20, 0x7E),
    UnicodeInterval::new(0x100, 0x17F),
    UnicodeInterval::new(0xC0, 0xFF),
    UnicodeInterval::new(0x1E00, 0x1EFF),
];
static CYRILLIC: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x400, 0x4FF),
    UnicodeInterval::new(0x500, 0x52F),
    UnicodeInterval::new(0x2DE0, 0x2DFF),
    UnicodeInterval::new(0xA640, 0xA69F),
];
static GREEK: &[UnicodeInterval] = &[UnicodeInterval::new(0x370, 0x3FF), UnicodeInterval::new(0x1F00, 0x1FFF)];
static ARABIC: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x600, 0x6FF),
    UnicodeInterval::new(0x750, 0x77F),
    UnicodeInterval::new(0x8A0, 0x8FF),
    UnicodeInterval::new(0xFB50, 0xFDFF),
    UnicodeInterval::new(0xFE70, 0xFEFF),
];
static MATH: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x2200, 0x22FF),
    UnicodeInterval::new(0x27C0, 0x27EF),
    UnicodeInterval::new(0x2980, 0x29FF),
    UnicodeInterval::new(0x2A00, 0x2AFF),
];
static SUPERSCRIPTS: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x2070, 0x2070),
    UnicodeInterval::new(0x00B9, 0x00B9),
    UnicodeInterval::new(0x00B2, 0x00B3),
    UnicodeInterval::new(0x2074, 0x207F),
    UnicodeInterval::new(0x2071, 0x2071),
];
static NUMBERS: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x30, 0x39),
    UnicodeInterval::new(0x2150, 0x218F),
    UnicodeInterval::new(0x2460, 0x24FF),
];
static PICTOGRAPHS: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x2600, 0x27BF),
    UnicodeInterval::new(0x1F300, 0x1F5FF),
    UnicodeInterval::new(0x1F900, 0x1F9FF),
    UnicodeInterval::new(0x1FA70, 0x1FAFF),
];
static GAMES: &[UnicodeInterval] = &[
    UnicodeInterval::new(0x1F000, 0x1F0FF),
    UnicodeInterval::new(0x2654, 0x2667),
    UnicodeInterval::new(0x2680, 0x2685),
    UnicodeInterval::new(0x26C0, 0x26C3),
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
            UnicodeSymbols::Animals => Self::with_multiple_intervals(name, ANIMALS),
            UnicodeSymbols::Arrows => Self::with_multiple_intervals(name, ARROWS),
            UnicodeSymbols::Shapes => Self::with_multiple_intervals(name, SHAPES),
            UnicodeSymbols::Punctuation => Self::with_multiple_intervals(name, PUNCTUATION),
            UnicodeSymbols::Latin => Self::with_multiple_intervals(name, LATIN),
            UnicodeSymbols::Cyrillic => Self::with_multiple_intervals(name, CYRILLIC),
            UnicodeSymbols::Greek => Self::with_multiple_intervals(name, GREEK),
            UnicodeSymbols::Arabic => Self::with_multiple_intervals(name, ARABIC),
            UnicodeSymbols::Chinese => Self::with_interval(name, 0x4E00, 0x9FFF).unwrap(),
            UnicodeSymbols::Math => Self::with_multiple_intervals(name, MATH),
            UnicodeSymbols::Subscripts => Self::with_interval(name, 0x2080, 0x209C).unwrap(),
            UnicodeSymbols::Superscripts => Self::with_multiple_intervals(name, SUPERSCRIPTS),
            UnicodeSymbols::Numbers => Self::with_multiple_intervals(name, NUMBERS),
            UnicodeSymbols::Pictographs => Self::with_multiple_intervals(name, PICTOGRAPHS),
            UnicodeSymbols::Transport => Self::with_interval(name, 0x1F680, 0x1F6FF).unwrap(),
            UnicodeSymbols::Unicode => Self::with_interval(name, 0x0020, 0x10FFFF).unwrap(),
            UnicodeSymbols::Games => Self::with_multiple_intervals(name, GAMES),
        }
    }
    fn with_multiple_intervals(name: &str, mi: &'static [UnicodeInterval]) -> Self {
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
