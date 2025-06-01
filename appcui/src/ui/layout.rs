//! A layout system for positioning and sizing UI controls.
//!
//! The Layout component provides tools for arranging controls within containers.
//! It supports anchoring, docking, proportional sizing, and alignment options.

mod absolute_layout;
mod alignament;
mod all_anchors_layout;
mod anchors;
mod control_layout;
mod coordonate;
mod coordonate16;
mod dimension;
mod dimension16;
mod layout_mode;
mod layout_parameters;
mod left_bottom_right_anchors_layout;
mod left_right_anchors_layout;
mod left_top_right_anchors_layout;
mod macros;
mod parameter;
mod point_and_size_layout;
#[cfg(test)]
mod tests;
mod top_bottom_anchors_layout;
mod top_left_bottom_anchors_layout;
mod top_right_bottom_anchors_layout;

use absolute_layout::AbsoluteLayout;
use alignament::Alignament;
use all_anchors_layout::AllAnchorsLayout;
use anchors::Anchors;
pub(crate) use control_layout::ControlLayout;
pub use control_layout::Layout;
pub use coordonate::Coordonate;
pub(crate) use coordonate16::Coordonate16;
pub use dimension::Dimension;
pub(crate) use dimension16::Dimension16;
use layout_mode::LayoutMode;
use layout_parameters::LayoutParameters;
use left_bottom_right_anchors_layout::LeftBottomRightAnchorsLayout;
use left_right_anchors_layout::LeftRightAnchorsLayout;
use left_top_right_anchors_layout::LeftTopRightAnchorsLayout;
use macros::should_not_use;
use parameter::Parameter;
use point_and_size_layout::PointAndSizeLayout;
use top_bottom_anchors_layout::TopBottomAnchorsLayout;
use top_left_bottom_anchors_layout::TopLeftBottomAnchorsLayout;
use top_right_bottom_anchors_layout::TopRightBottomAnchorsLayout;
