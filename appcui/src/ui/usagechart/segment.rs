use crate::graphics::CharAttribute;

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

pub struct Segment {
    pub(crate) order: u16,
    pub(crate) poz: i32,
    pub(crate) value: u64,
    pub(crate) len: u16,
    pub(crate) label: String,
    pub(crate) char: SegmentFillType,
    pub(crate) attr: Option<CharAttribute>,
}