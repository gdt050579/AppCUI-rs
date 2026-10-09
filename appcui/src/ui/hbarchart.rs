//! A horizontal bar chart UI control for displaying numeric series.
//!
//! [`HBarChart`] stores one bar per value of type `T` (any [`Number`](crate::ui::common::Number)).
//! The horizontal axis holds the values. The Y axis holds the bar labels.
//!
//! [`Flags`] selects scroll bars, a zero line, and dimming of unselected bars.
//! [`BarScale`] maps values onto bar length. [`YAxisLabelMode`] chooses the labels
//! beside the bars. [`events::GenericHBarChartEvents`] reports selection changes.
//! [`Bar`], [`BarBuilder`], and [`BarSpan`] describe individual bars and label groups.

pub mod events;
mod hbarchart;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::hbarchart::Bars;
pub use self::hbarchart::HBarChart;
pub use self::initialization_flags::BarScale;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::YAxisLabelMode;

use self::initialization_flags::YAxisLabelFormat;

pub use super::components::Bar;
pub use super::components::BarBuilder;
pub use super::components::BarCapType;
pub use super::components::BarDrawMode;
pub use super::components::BarFillType;
pub use super::components::BarLargePointType;
pub use super::components::BarPointType;
pub use super::components::BarSpan;
