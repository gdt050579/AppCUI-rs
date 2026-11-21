use crate::graphics::CharAttribute;

/// Character repeated to fill a segment of a [`struct@super::UsageChart`].
///
/// [`Solid`](Self::Solid) is the default, used by a segment built from a number.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SegmentFillType {
    /// U+2588 █ — full block.
    Solid,
    /// U+2593 ▓ — dense shade.
    Shade75,
    /// U+2592 ▒ — medium shade.
    Shade50,
    /// U+2591 ░ — light shade.
    Shade25,
    /// U+28FF ⣿ — full 2×4 braille cell.
    Braille,
    /// U+259A ▚ — upper-left and lower-right quadrants. Repeated, this tiles as a checkerboard.
    Checkerboard,
    /// U+253C ┼ — light grid.
    Grid,
    /// U+256C ╬ — double-line grid, heavier than [`Grid`](Self::Grid).
    GridDouble,
    /// U+2573 ╳ — diagonal cross-hatch.
    CrossHatch,
    /// U+2507 ┇ — heavy triple-dash vertical line.
    Dashed,
    /// U+2571 ╱ — diagonal rising from left to right.
    DiagonalUp,
    /// U+2572 ╲ — diagonal falling from left to right.
    DiagonalDown,
    /// U+25A0 ■ — black square.
    Notched,
    /// Fills the segment with this character.
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
            SegmentFillType::Checkerboard => '\u{259A}',
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

/// One part of a [`struct@super::UsageChart`].
///
/// A segment has a `u64` value, a label, a [`SegmentFillType`], and an optional color.
/// Build one with [`SegmentBuilder`], or convert a `u8`, `u16`, `u32`, or `u64`.
/// A number becomes a solid segment with that value and an empty label.
/// A segment without a color is drawn with the next automatic bar color.
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
/// Builds a [`Segment`].
///
/// # Example
/// ```rust
/// use appcui::prelude::*;
///
/// let segment = usagechart::SegmentBuilder::new()
///     .value(48)
///     .label("OS")
///     .fill(usagechart::SegmentFillType::Solid)
///     .attr(CharAttribute::with_fore_color(Color::Red))
///     .build();
/// ```
pub struct SegmentBuilder {
    segment: Segment,
}
impl SegmentBuilder {
    /// Starts a segment with value `0`, an empty label, a solid fill, and no color.
    pub fn new() -> Self {
        Self { segment: Segment::with_value(0) }
    }
    /// Sets the value of the segment.
    pub fn value(mut self, value: u64) -> Self {
        self.segment.value = value;
        self
    }
    /// Sets the label of the segment. An empty label is allowed.
    pub fn label(mut self, label: &str) -> Self {
        self.segment.label = label.to_string();
        self
    }
    /// Sets the character that fills the segment.
    pub fn fill(mut self, fill: SegmentFillType) -> Self {
        self.segment.fill_type = fill;
        self
    }
    /// Sets the color of the segment. Without this, the chart picks an automatic bar color.
    pub fn attr(mut self, attr: CharAttribute) -> Self {
        self.segment.attr = Some(attr);
        self
    }
    /// Returns the finished segment.
    pub fn build(self) -> Segment {
        self.segment
    }
}
/// A solid segment with this value and an empty label.
impl From<u8> for Segment {
    fn from(value: u8) -> Self {
        Segment::with_value(value as u64)
    }
}
/// A solid segment with this value and an empty label.
impl From<u16> for Segment {
    fn from(value: u16) -> Self {
        Segment::with_value(value as u64)
    }
}
/// A solid segment with this value and an empty label.
impl From<u32> for Segment {
    fn from(value: u32) -> Self {
        Segment::with_value(value as u64)
    }
}
/// A solid segment with this value and an empty label.
impl From<u64> for Segment {
    fn from(value: u64) -> Self {
        Segment::with_value(value)
    }
}