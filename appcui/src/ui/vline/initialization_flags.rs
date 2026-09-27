use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::VLine`].
///
/// Combine values with `|`. `Flags::None` draws a single vertical line (`│`) that
/// is not joined to neighboring borders.
pub enum Flags {
    /// Draw a double vertical line (`║`) instead of a single one.
    DoubleLine = 0x0001,
    /// Join the line with neighboring window or panel borders.
    MergeBorders = 0x0002,
}