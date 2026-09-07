use crate::prelude::*;
use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits: 8)]
pub enum Flags {
    ScrollBars = 1,
    SearchBar = 0x02,
    MultiSelect = 0x04,
}

/// How edges are drawn between nodes in a [`super::GraphView`]: straight lines or orthogonal segments.
#[derive(Clone, Copy, PartialEq, Eq, EnumSelector)]
pub enum EdgeRouting {
    #[VariantInfo(name = "Direct", description = "Draw edges as direct lines between nodes")]
    Direct,
    #[VariantInfo(name = "Orthogonal", description = "Draw edges as orthogonal lines between nodes")]
    Orthogonal,
}

#[derive(Copy, Clone, Eq, PartialEq)]
/// Layout algorithm used to position nodes in a [`super::GraphView`].
///
/// Choose an unchanged layout, grid, circular, hierarchical, or force-directed
/// arrangement. Packed variants reduce unused space between nodes.
pub enum ArrangeMethod {
    None,
    Grid,
    GridPacked,
    Circular,
    Hierarchical,
    HierarchicalPacked,
    ForceDirected,
}
