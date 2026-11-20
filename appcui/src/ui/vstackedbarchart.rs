//! A vertical stacked bar chart UI control.
//!
//! `VStackedBarChart` draws one column per [`BarsStack`] of type `T`
//! (any [`Number`](crate::ui::common::Number)). Positive segments stack upward from zero
//! and negative segments stack downward.

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
