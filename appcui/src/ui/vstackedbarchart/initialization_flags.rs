use EnumBitFlags::EnumBitFlags;

use crate::ui::components::BarSpan;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::VStackedBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the stacks without extra decorations.
pub enum Flags {
    /// Shows a horizontal scroll bar when the stacks are wider than the control.
    ///
    /// The bottom margin grows while the chart has focus so the scroll bar stays visible.
    ScrollBars = 1,
    /// Draws the chart, except the selected stack or segment, with the inactive chart color.
    DimBarsOnSelection = 2,
    /// Draws a solid horizontal line at value zero while the Y-axis grid is visible.
    ShowZeroLineOnYAxis = 4,
}

/// Chooses the labels drawn under the stacks of a [`struct@super::VStackedBarChart`].
pub enum XAxisLabelMode<'a> {
    /// Draws no X axis.
    None,
    /// Labels each stack with `start + stack index`.
    ///
    /// The first stack is labeled `start`, the next `start + 1`, and so on.
    Index(i32),
    /// Uses each stack's own label. Stacks with an empty label are skipped.
    StackLabels,
    /// Labels ranges of stacks.
    ///
    /// Each [`BarSpan`] covers a run of stacks. The spans are copied into the chart,
    /// ordered by start index and then by end index. A span that overlaps an earlier one is dropped.
    Custom(&'a [BarSpan]),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum XAxisLabelFormat {
    None,
    Index(i32),
    StackLabels,
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
            _ => 2,
        }
    }
}
