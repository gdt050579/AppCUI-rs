use EnumBitFlags::EnumBitFlags;

use crate::ui::common::Number;

#[EnumBitFlags(bits = 8)]
/// Initialization flags for a [`struct@super::VBarChart`].
///
/// Combine values with `|`. `Flags::None` draws the bars without extra decorations.
pub enum Flags {}



pub enum BarScale<T: Number + 'static> {
    Auto,
    FromZero,
    FitData,
    Fixed { min: T, max: T },
}