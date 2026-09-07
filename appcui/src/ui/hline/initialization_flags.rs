use EnumBitFlags::EnumBitFlags;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for an [`struct@super::HLine`].
///
/// Combine values with `|`. `Flags::None` draws a single horizontal line (`─`)
/// without a title and without joining neighboring borders.
pub enum Flags {
    /// Draw a double horizontal line (`═`) instead of a single one.
    DoubleLine = 0x0001,
    /// Center the constructor title on the line, for example `─ Title ─`.
    HasTitle = 0x0002,
    /// Join the line with neighboring window or panel borders.
    MergeBorders = 0x0004,
}