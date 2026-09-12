use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for a [`struct@super::Selector`].
///
/// Combine values with `|`. `Flags::None` requires a concrete enum variant at all times.
pub enum Flags {
    /// Allow clearing the selection so that no variant is chosen.
    AllowNoneVariant = 0x01,
}

/// How a [`struct@super::Selector`] enumerates and displays enum variants.
///
/// Types that derive `EnumSelector` implement this automatically. `COUNT` is the
/// number of variants, [`from_index`](Self::from_index) maps `0..COUNT`, and
/// [`name`](Self::name) / [`description`](Self::description) supply the closed-control text.
pub trait EnumSelector {
    /// Number of selectable variants.
    const COUNT: u32;
    /// Returns the variant at `index`, or `None` if `index` is out of range.
    fn from_index(index: u32) -> Option<Self> where Self: Sized;
    /// Short display name of this variant.
    fn name(&self) -> &'static str;
    /// Optional longer description; defaults to an empty string.
    fn description(&self) -> &'static str {
        ""
    }
}
