use crate::ui::components::BarSpan;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::LineChart`].
///
/// Combine values with `|`. `Flags::None` draws the chart without extra decorations.
pub enum Flags {
    /// Draws a solid horizontal line at value zero while the Y-axis grid is visible.
    ShowZeroLineOnYAxis = 1,
}

/// Chooses the labels drawn under the points of a [`struct@super::LineChart`].
pub enum XAxisLabelMode<'a> {
    /// Draws no X axis.
    None,
    /// Labels each point with `start + point index`.
    Index(i32),
    /// Labels each point with its own label.
    ///
    /// Because all series share the X axis, the label at an index is the first
    /// non-empty label found among the series at that index. Indexes with no label are skipped.
    PointLabels,
    /// Labels ranges of points.
    ///
    /// Each [`BarSpan`] covers a run of points. The spans are copied into the chart,
    /// ordered by start index and then by end index. A span that overlaps an earlier one is dropped.
    Custom(&'a [BarSpan]),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum XAxisLabelFormat {
    None,
    Index(i32),
    PointLabels,
    Custom,
}
impl XAxisLabelFormat {
    #[inline(always)]
    pub(super) fn is_none(&self) -> bool {
        matches!(self, XAxisLabelFormat::None)
    }
    #[inline(always)]
    pub(super) fn height(&self) -> u8 {
        match self {
            XAxisLabelFormat::None => 0,
            _ => 2,
        }
    }
}
