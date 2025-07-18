#[derive(Copy, Clone)]
pub enum ColorSchema {
    Auto,
    Color16,
    #[cfg(feature = "TRUE_COLORS")]
    TrueColors,
    GrayScale4,
    #[cfg(feature = "TRUE_COLORS")]
    GrayScaleTrueColors,
    BlackAndWhite,
}