//! A vertical bar chart UI control for displaying numeric series.
//!
//! `VBarChart` draws one vertical bar per value of type `T` (any [`Number`](crate::ui::common::Number)).

mod vbarchart;
mod initialization_flags;
pub mod events;
#[cfg(test)]
mod tests;

pub use self::vbarchart::VBarChart;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::BarScale;
pub use self::initialization_flags::XAxisLabelMode;

use self::initialization_flags::XAxisLabelFormat;
use super::components::BarLayout;
use super::components::BarDefaults;

pub use super::components::BarDrawMode;
pub use super::components::BarBuilder;
pub use super::components::Bar;
pub use super::components::BarFillType;
pub use super::components::BarCapType;
pub use super::components::BarPointType;
pub use super::components::BarLargePointType;
pub use super::components::BarSpan;
pub use super::components::Bars;
