/// Palette used when converting image pixels to terminal [`Color`](crate::graphics::Color)s.
///
/// [`Auto`](Self::Auto) picks 24-bit color when the `TRUE_COLORS` feature is enabled,
/// otherwise the 16-color console palette.
#[derive(Copy, Clone)]
pub enum ColorSchema {
    /// Use true color when available, otherwise the 16-color palette.
    Auto,
    /// The 16 standard console colors (`Black` … `White`).
    Color16,
    /// 24-bit RGB colors (requires the `TRUE_COLORS` feature).
    #[cfg(feature = "TRUE_COLORS")]
    TrueColors,
    /// Four gray levels (`Black`, `Gray`, `Silver`, `White`).
    GrayScale4,
    /// Grayscale RGB values (requires the `TRUE_COLORS` feature).
    #[cfg(feature = "TRUE_COLORS")]
    GrayScaleTrueColors,
    /// Only black and white, using the render options luminance threshold.
    BlackAndWhite,
}