use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits=8)]
/// Initialization flags for an [`struct@super::Accordion`].
///
/// Combine values with `|`. `Flags::None` keeps the default look, where the accordion
/// paints its own background.
pub enum Flags {
    /// Do not fill the accordion background, so the parent control shows through.
    TransparentBackground = 0x01,
}