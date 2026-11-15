//! A vertical stacked bar chart UI control for displaying numeric series.
//!
//! `VStackedBarChart` is generic over any [`Number`](crate::ui::common::Number) type.
//! This is currently only a skeleton: it has no data, painting or events yet.

mod vstackedbarchart;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::vstackedbarchart::VStackedBarChart;
pub use self::initialization_flags::Flags;
