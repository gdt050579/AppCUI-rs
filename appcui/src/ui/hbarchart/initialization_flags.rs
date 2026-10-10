use EnumBitFlags::EnumBitFlags;

use crate::ui::hbarchart::BarSpan;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::HBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the bars without extra decorations.
pub enum Flags {
    /// Shows a vertical scroll bar when the bars are taller than the control.
    ///
    /// The right margin grows while the chart has focus so the scroll bar stays visible.
    ScrollBars = 1,
    /// Draws every bar except the selected one with the inactive chart color.
    DimBarsOnSelection = 2,
    /// Draws a solid vertical line at value zero, labeled `0`, while the X-axis grid is visible.
    ShowZeroLineOnXAxis = 4,
}

/// Chooses the labels drawn beside the bars of an [`struct@super::HBarChart`].
pub enum YAxisLabelMode<'a> {
    /// Draws no category axis.
    None,
    /// Labels each bar with `start + bar index`.
    ///
    /// The first bar is labeled `start`, the next `start + 1`, and so on.
    Index(i32),
    /// Uses each bar's own label. Bars with an empty label are skipped.
    BarLabels,
    /// Labels ranges of bars.
    ///
    /// Each [`BarSpan`] covers a run of bars. The spans are copied into the chart,
    /// ordered by start index and then by end index. A span that overlaps an earlier one is dropped.
    Custom(&'a [BarSpan]),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum YAxisLabelFormat {
    None,
    Index(i32),
    BarLabels,
    Custom,
}
#[allow(dead_code)]
impl YAxisLabelFormat {
    #[inline(always)]
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, YAxisLabelFormat::None)
    }
    pub(crate) fn width(&self) -> u8 {
        match self {
            YAxisLabelFormat::None => 0,
            _ => 2,
        }
    }
}
