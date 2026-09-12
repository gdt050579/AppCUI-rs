use crate::prelude::*;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits: 8)]
/// Initialization flags for a [`struct@super::GraphView`].
///
/// Combine values with `|`. `Flags::None` shows the graph without scroll bars,
/// search, or multi-select UI.
pub enum Flags {
    /// Show scroll bars for navigating a graph larger than the view.
    ScrollBars = 1,
    /// Show a search bar for finding nodes by name.
    SearchBar = 0x02,
    /// Enable a multi-select gutter so several nodes can be checked at once.
    MultiSelect = 0x04,
}

/// How edges are drawn between nodes in a [`super::GraphView`]: straight lines or orthogonal segments.
#[derive(Clone, Copy, PartialEq, Eq, EnumSelector)]
pub enum EdgeRouting {
    /// Straight line between node centers, for example `A────────B`.
    #[VariantInfo(name = "Direct", description = "Draw edges as direct lines between nodes")]
    Direct,
    /// Horizontal and vertical segments, for example `A──┐` / `   └──B`.
    #[VariantInfo(name = "Orthogonal", description = "Draw edges as orthogonal lines between nodes")]
    Orthogonal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// Layout algorithm used to position nodes in a [`super::GraphView`].
///
/// Choose an unchanged layout, grid, circular, hierarchical, or force-directed
/// arrangement. Packed variants reduce unused space between nodes.
pub enum ArrangeMethod {
    /// Leave node positions unchanged.
    None,
    /// Place nodes on a regular grid.
    Grid,
    /// Pack nodes on a grid with less unused space.
    GridPacked,
    /// Arrange nodes around a circle.
    Circular,
    /// Tree-like layers from a root.
    Hierarchical,
    /// Hierarchical layout with less unused space.
    HierarchicalPacked,
    /// Spring/force simulation that spreads connected nodes.
    ForceDirected,
}
