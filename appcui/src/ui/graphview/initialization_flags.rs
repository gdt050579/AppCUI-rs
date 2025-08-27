use EnumBitFlags::EnumBitFlags;
use crate::prelude::*;

#[EnumBitFlags(bits: 8)]
pub enum Flags {
    ScrollBars = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumSelector)]
pub enum EdgeRouting {
    #[VariantInfo(name = "Direct", description = "Draw edges as direct lines between nodes")]
    Direct,
    #[VariantInfo(name = "Orthogonal", description = "Draw edges as orthogonal lines between nodes")]
    Orthogonal,
}