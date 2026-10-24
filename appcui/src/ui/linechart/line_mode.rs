use crate::graphics::{LineType, OrthogonalDirection};

/// How a [`struct@super::LineChart`] connects consecutive points of a series.
///
/// Lines are drawn first and the point markers are drawn on top of them, so a line always
/// goes from the center of one marker to the center of the next one.
/// [`Line`](Self::Line) with [`LineType::Single`] is the default.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineMode {
    /// Points are not connected (scatter plot).
    None,
    /// Connects points with a Braille line (2 by 4 dots per cell).
    Braille,
    /// Connects points with a direct (possibly diagonal) line drawn with the given [`LineType`].
    Line(LineType),
    /// Connects points with horizontal and vertical segments only (step chart).
    ///
    /// [`OrthogonalDirection`] chooses how each segment bends.
    Orthogonal(LineType, OrthogonalDirection),
}

impl Default for LineMode {
    fn default() -> Self {
        LineMode::Line(LineType::Single)
    }
}
