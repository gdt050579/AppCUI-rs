use EnumBitFlags::EnumBitFlags;

use crate::ui::vbarchart::BarSpan;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::VBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the bars without extra decorations.
pub enum Flags {
    /// Shows a horizontal scroll bar when the bars are wider than the control.
    ///
    /// The bottom margin grows while the chart has focus so the scroll bar stays visible.
    ScrollBars = 1,
    /// Draws every bar except the selected one with the inactive chart color.
    DimBarsOnSelection = 2,
    /// Draws a solid horizontal line at value zero while the Y-axis grid is visible.
    ShowZeroLineOnYAxis = 4,
}

/// Chooses the labels drawn under the bars of a [`struct@super::VBarChart`].
pub enum XAxisLabelMode<'a> {
    /// Draws no X axis.
    None,
    /// Labels each bar with `start + bar index`.
    ///
    /// The first bar is labeled `start`, the next `start + 1`, and so on.
    Index (i32),
    /// Uses each bar's own label. Bars with an empty label are skipped.
    BarLabels,
    /// Labels ranges of bars.
    ///
    /// Each [`BarSpan`] covers a run of bars. The spans are copied into the chart,
    /// ordered by start index and then by end index. A span that overlaps an earlier one is dropped.
    Custom(&'a [BarSpan]),
}

#[derive(Copy,Clone, PartialEq, Eq)]
pub(super) enum XAxisLabelFormat {
    None,
    Index (i32),
    BarLabels,
    Custom,
}
impl XAxisLabelFormat {
    #[inline(always)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, XAxisLabelFormat::None)
    }
    pub(crate) fn height(&self) -> u8 {
        match self {
            XAxisLabelFormat::None => 0,
            _ => 2
        }
    }
}