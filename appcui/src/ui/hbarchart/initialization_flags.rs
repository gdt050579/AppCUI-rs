use EnumBitFlags::EnumBitFlags;

use crate::ui::{common::Number, hbarchart::BarSpan};

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::HBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the bars without extra decorations.
pub enum Flags {
    /// Shows a scroll bar when the bars do not fit in the control.
    ///
    /// The margin grows while the chart has focus so the scroll bar stays visible.
    ScrollBars = 1,
    /// Draws every bar except the selected one with the inactive chart color.
    DimBarsOnSelection = 2,
    /// Draws a solid line at value zero while the X-axis grid is visible.
    ShowZeroLineOnXAxis = 4,
}

/// How bar values are mapped onto the length of a bar in an [`struct@super::HBarChart`].
pub enum BarScale<T: Number + 'static> {
    /// Draws every bar from zero. The visible range includes zero and every bar value.
    FromZero,
    /// Draws every bar from zero, and expands the range so that it covers `min`, `max`, and every bar value.
    FromZeroMinRange {
        /// Lowest value that must remain inside the scale.
        min: T,
        /// Highest value that must remain inside the scale.
        max: T,
    },
    /// Stretches the smallest and largest bar values across the full plot length.
    ///
    /// When every value is equal, each bar is drawn at half the plot length.
    FitData,
    /// Uses a fixed range. Values outside it are drawn at the corresponding edge of the plot.
    ///
    /// When `min` is not lower than `max`, every bar is drawn at half the plot length.
    Fixed {
        /// Lowest value represented by the start of the plot.
        min: T,
        /// Highest value represented by the end of the plot.
        max: T,
    },
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
