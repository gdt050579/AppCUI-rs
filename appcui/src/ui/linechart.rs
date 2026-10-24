//! A line chart UI control that plots one or more series as points connected by line segments.
//!
//! `LineChart` draws every [`Series`] over a shared X axis. Values can be of any type `T`
//! that implements [`Number`](crate::ui::common::Number); they are converted to an integer
//! internally for rendering. Each point is drawn with a [`Marker`], and consecutive points
//! of a series are connected according to its [`LineMode`].

pub mod events;
mod initialization_flags;
mod line_mode;
mod linechart;
mod marker;
mod series;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::initialization_flags::XAxisLabelMode;
pub use self::line_mode::LineMode;
pub use self::linechart::LineChart;
pub use self::marker::Marker;
pub use self::series::{Series, SeriesBuilder};

pub use super::components::Bar;
pub use super::components::BarBuilder;
pub use super::components::BarDrawMode;
pub use super::components::BarFramedPointType;
pub use super::components::BarPointType;
pub use super::components::BarScale;
pub use super::components::BarSpan;

use self::initialization_flags::XAxisLabelFormat;
