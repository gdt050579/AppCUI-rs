use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::PathFinder`].
///
/// Combine values with `|`. `Flags::None` is an editable path with case-insensitive
/// matching while navigating.
pub enum Flags {
    /// Display the path but do not allow editing or navigation.
    ReadOnly = 0x01,
    /// Match path components case-sensitively while navigating.
    CaseSensitive = 0x02
}