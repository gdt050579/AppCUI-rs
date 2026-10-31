use crate::graphics::CharAttribute;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SegmentFillType {
    /// U+2588 — full block.
    Solid,
    /// U+2593 ▓ — dense shade.
    Shade75,
    /// U+2592 ▒ — medium shade.
    Shade50,
    /// U+2591 ░ — light shade.
    Shade25,
    /// 2x4 braille cells used as a fill.
    Braille,
    /// 2x4 checkerboard pattern.
    Checkerboard,
    /// U+253C ┼ — light grid/lattice.
    Grid,
    /// U+256C ╬ — double-line grid, heavier than [`Grid`](Self::Grid).
    GridDouble,
    /// U+2573 ╳ — diagonal cross-hatch.
    CrossHatch,
    /// ┇ — dashed line.
    Dashed,
    /// U+2571 ╱ — forward diagonal (rising left-to-right).
    DiagonalUp,
    /// U+2572 ╲ — backward diagonal (falling left-to-right).
    DiagonalDown,
    /// U+2587 ▇ — near-solid block with a thin notch (gap) at the top of each cell.
    Notched,
    /// Fill with an arbitrary character.
    Custom(char),
}

impl SegmentFillType {
    pub(super) fn as_char(&self) -> char {
        match self {
            SegmentFillType::Solid => '█',
            SegmentFillType::Shade75 => '▓',
            SegmentFillType::Shade50 => '▒',
            SegmentFillType::Shade25 => '░',
            SegmentFillType::Braille => '⣿',
            SegmentFillType::Checkerboard => '⣿',
            SegmentFillType::Grid => '┼',
            SegmentFillType::GridDouble => '╬',
            SegmentFillType::CrossHatch => '╳',
            SegmentFillType::Dashed => '┇',
            SegmentFillType::DiagonalUp => '╱',
            SegmentFillType::DiagonalDown => '╲',
            SegmentFillType::Notched => '■',
            SegmentFillType::Custom(ch) => *ch,
        }
    }
}

pub struct Segment {
    pub(crate) order: u16,
    pub(crate) poz: i32,
    pub(crate) value: u64,
    pub(crate) len: u16,
    pub(crate) label: String,
    pub(crate) fill_type: SegmentFillType,
    pub(crate) attr: Option<CharAttribute>,
}
impl Segment {
    fn with_value(value: u64) -> Self {
        Self {
            order: 0,
            poz: 0,
            value,
            len: 0,
            label: String::new(),
            fill_type: SegmentFillType::Solid,
            attr: None,
        }
    }
    pub(super) fn range(&self) -> (i32, i32) {
        (self.poz, self.poz + self.len as i32)
    }
}
pub struct SegmentBuilder {
    segment: Segment,
}
impl SegmentBuilder {
    pub fn new() -> Self {
        Self { segment: Segment::with_value(0) }
    }
    pub fn value(mut self, value: u64) -> Self {
        self.segment.value = value;
        self
    }
    pub fn label(mut self, label: &str) -> Self {
        self.segment.label = label.to_string();
        self
    }
    pub fn fill(mut self, fill: SegmentFillType) -> Self {
        self.segment.fill_type = fill;
        self
    }
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.segment.attr = Some(attr);
        self
    }
    pub fn build(self) -> Segment {
        self.segment
    }
}
impl From<u8> for Segment {
    fn from(value: u8) -> Self {
        Segment::with_value(value as u64)
    }
}
impl From<u16> for Segment {
    fn from(value: u16) -> Self {
        Segment::with_value(value as u64)
    }
}
impl From<u32> for Segment {
    fn from(value: u32) -> Self {
        Segment::with_value(value as u64)
    }
}
impl From<u64> for Segment {
    fn from(value: u64) -> Self {
        Segment::with_value(value)
    }
}