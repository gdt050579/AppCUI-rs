//! A line chart UI control that plots one or more series as points connected by line segments.
//!
//! `LineChart` draws every series over a shared category (X) axis. Values can be of any
//! type `T` that implements [`Number`](crate::ui::common::Number); they are converted to an
//! integer internally for rendering.

mod linechart;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::linechart::LineChart;
pub use self::initialization_flags::Flags;
