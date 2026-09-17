//! A vertical bar chart UI control for displaying numeric series.
//!
//! `VBarChart` draws one vertical bar per value of type `T` (any [`Number`](crate::ui::common::Number)).

mod vbarchart;
mod initialization_flags;
pub mod events;
mod bar;
#[cfg(test)]
mod tests;

pub use self::vbarchart::VBarChart;
pub use self::initialization_flags::Flags;
pub use self::initialization_flags::BarScale;
pub use self::bar::BarDrawMode;
pub use self::bar::BarBuilder;
pub use self::bar::Bar;
