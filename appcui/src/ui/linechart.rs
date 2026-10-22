//! A line chart UI control that plots one or more series as points connected by line segments.
//!
//! `LineChart` draws every [`Series`] over a shared X axis. Values can be of any type `T`
//! that implements [`Number`](crate::ui::common::Number); they are converted to an integer
//! internally for rendering. Each point is drawn with a [`Marker`], and consecutive points
//! of a series are connected according to its [`LineMode`].

mod initialization_flags;
mod line_mode;
mod linechart;
mod marker;
mod series;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::line_mode::LineMode;
pub use self::linechart::LineChart;
pub use self::marker::Marker;
pub use self::series::{Series, SeriesBuilder};
