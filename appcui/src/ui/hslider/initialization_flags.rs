use EnumBitFlags::EnumBitFlags;

#[repr(u8)]
#[derive(Copy,Clone,PartialEq,Eq)]
/// Visual style of an [`super::HSlider`] track and marker.
///
/// `Standard` uses a boxed track, `ProgressBar` fills from the left, `Inline` and
/// `Blocks` use Unicode bar characters, and `Ruler` draws tick marks below the track.
pub enum Type {
    /// Boxed track with a marker, for example `[....X....]`.
    Standard,
    /// Fill from the left like a progress bar, for example `[====>    ]`.
    ProgressBar,
    /// Thin Unicode bar with a round marker, for example `━━━━●━━━━`.
    Inline,
    /// Solid/empty blocks, for example `█████░░░░░`.
    Blocks,
    /// Bar with tick marks underneath, for example `━━━━●━━━━` over `┕━━┷━━┙`.
    Ruler,
}

impl Type {
    pub (super) fn char_set(&self ) -> &'static CharSet {
       match &self {
            Type::Standard => &STANDARD,
            Type::ProgressBar => &PROGRESS_BAR,
            Type::Inline => &INLINE,
            Type::Blocks => &BLOCKS,
            Type::Ruler => &RULER,
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

static BLOCKS: CharSet = CharSet {
    marker: '█',
    left_marker: None,
    right_marker: None,
    left_marker_line: '█',
    right_marker_line: '░',
    left_cap: None,
    right_cap: None,
    tick: '│',
    left_tick: '│',
    right_tick: '│',
};

static RULER: CharSet = CharSet {
    marker: '●',
    left_marker: None,
    right_marker: None,
    left_marker_line: '━',
    right_marker_line: '━',
    left_cap: None,
    right_cap: None,
    tick: '┷',
    left_tick: '┕',
    right_tick: '┙',
};

#[EnumBitFlags(bits=16)]
/// Initialization flags for an [`struct@super::HSlider`].
///
/// Combine values with `|`. `Flags::None` draws the track and marker without a
/// numeric label or tick marks.
pub enum Flags {
    /// Show the current value next to the track.
    ShowValue     = 0x0001,
    /// Draw tick marks along the track.
    Ticks         = 0x0002,
    /// Replace the marker glyph with the numeric value.
    ValueAsMarker = 0x0004,
}
