use EnumBitFlags::EnumBitFlags;

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Type {
    Standard,
    ProgressBar,
    Inline,
}

impl Type {
    pub (super) fn char_set(&self ) -> &'static CharSet {
       match &self {
           Type::Standard => &STANDARD,
           Type::ProgressBar => &PROGRESS_BAR,
           Type::Inline => &INLINE,
       } 
    }
}

pub (super) struct CharSet {
    pub (super) marker: char,
    pub (super) left_marker: Option<char>,
    pub (super) right_marker: Option<char>,
    pub (super) left_marker_line: char,
    pub (super) right_marker_line: char,
    pub (super) left_cap: Option<char>,
    pub (super) right_cap: Option<char>, 
    pub (super) tick: char,
    pub (super) left_tick: char,
    pub (super) right_tick: char,
}

static STANDARD: CharSet = CharSet {
    marker: 'X',
    left_marker: Some('['),
    right_marker: Some(']'),
    left_marker_line: '.',
    right_marker_line: '.',
    left_cap: Some('['),
    right_cap: Some(']'),
    tick: '|',
    left_tick: '|',
    right_tick: '|',
};

static PROGRESS_BAR: CharSet = CharSet {
    marker: '>',
    left_marker: None,
    right_marker: None,
    left_marker_line: '=',
    right_marker_line: ' ',
    left_cap: Some('['),
    right_cap: Some(']'),
    tick: '|',
    left_tick: '|',
    right_tick: '|',
};

static INLINE: CharSet = CharSet {
    marker: '●',
    left_marker: None,
    right_marker: None,
    left_marker_line: '━',
    right_marker_line: '━',
    left_cap: None,
    right_cap: None,
    tick: '┿',
    left_tick: '┝',
    right_tick: '┥',
};


#[EnumBitFlags(bits=16)]
pub enum Flags {
    ShowValue     = 0x0001,
    Ticks         = 0x0002,
    ValueAsMarker = 0x0004,
}
