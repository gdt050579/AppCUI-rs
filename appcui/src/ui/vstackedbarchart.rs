//! A vertical stacked bar chart UI control for displaying numeric series.
//!
//! `VStackedBarChart` is generic over any [`Number`](crate::ui::common::Number) type.
//! This is currently only a skeleton: it has no data, painting or events yet.

mod vstackedbarchart;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

use self::initialization_flags::XAxisLabelFormat;

pub use self::vstackedbarchart::VStackedBarChart;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::XAxisLabelMode;
pub use super::components::StackBar;
pub use super::components::StackBarBuilder;
pub use super::components::StackBarDrawMode;
pub use super::components::StackBarScale;
pub use super::components::BarsStack;
pub use super::components::BarsStackBuilder;
pub use super::components::BarFillType;
pub use super::components::BarSpan;
