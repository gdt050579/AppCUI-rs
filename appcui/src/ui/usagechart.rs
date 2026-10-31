//! A usage chart that draws one horizontal bar divided into segments.
//!
//! [`UsageChart`] stores one [`Segment`] per value. The width of a segment is its
//! share of the [`TotalUsage`]. [`Flags`] adds a margin around the bar and dims the
//! segments that are not selected. [`SortType`] orders the segments. [`LabelContent`]
//! chooses the hover tooltip. [`SegmentFillType`] chooses the character that fills
//! a segment. [`events::UsageChartEvents`] reports selection changes.
//!
//! Build a segment with [`SegmentBuilder`], or pass a `u8`, `u16`, `u32`, or `u64`
//! to [`UsageChart::add_segment`].

pub mod events;
mod segment;
mod usagechart;
mod initialization_flags;
#[cfg(test)]
mod tests;

pub use self::initialization_flags::Flags;
pub use self::initialization_flags::LabelContent;
pub use self::initialization_flags::SortType;
pub use self::initialization_flags::TotalUsage;
pub use self::usagechart::UsageChart;
pub use self::segment::SegmentFillType;
pub use self::segment::SegmentBuilder;
pub use self::segment::Segment;