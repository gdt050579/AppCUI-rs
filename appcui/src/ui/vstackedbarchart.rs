//! A vertical stacked bar chart UI control for displaying numeric series.
//!
//! `VStackedBarChart` is generic over any [`Number`](crate::ui::common::Number) type.
//! This is currently only a skeleton: it has no data, painting or events yet.

mod vstackedbarchart;
mod initialization_flags;
mod stackbar;
pub mod events;
#[cfg(test)]
mod tests;

use self::initialization_flags::XAxisLabelFormat;

pub use self::vstackedbarchart::VStackedBarChart;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::XAxisLabelMode;
pub use self::stackbar::StackBar;
pub use self::stackbar::StackBarBuilder;
pub use self::stackbar::StackBarDrawMode;
pub use self::stackbar::StackBarScale;
pub use self::stackbar::BarsStack;
pub use self::stackbar::BarsStackBuilder;
pub use super::components::BarFillType;
pub use super::components::BarSpan;

use self::vstackedbarchart::Selection;
