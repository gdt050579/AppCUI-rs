use EnumBitFlags::EnumBitFlags;
use crate::ui::common::NumberFormat;

#[EnumBitFlags(bits = 8)]
pub enum Flags {
    HideButtons = 0x0001,
    ReadOnly = 0x0002,
}


#[derive(Clone, Copy)]
/// How the value of a [`super::NumericSelector`] is displayed.
///
/// Choose plain decimal, a percentage, thousands grouping, hexadecimal, or a
/// human-readable size. This maps to [`crate::ui::common::NumberFormat`].
pub enum Format {
    /// Plain decimal, for example `1234`.
    Decimal,
    /// A percentage with a `%` suffix, for example `12%`.
    Percentage,
    /// Decimal with thousands separators, for example `1,234`.
    DigitGrouping,
    /// Hexadecimal, for example `0x4D2`.
    Hex,
    /// Human-readable size, for example `1 KB`.
    Size, 
}

impl From<Format> for NumberFormat {
    fn from(f: Format) -> Self {
        match f {
            Format::Decimal => NumberFormat::Decimal,
            Format::Percentage => NumberFormat::Percentage,
            Format::DigitGrouping => NumberFormat::DigitGrouping,
            Format::Hex => NumberFormat::Hex,
            Format::Size => NumberFormat::Size,
        }
    }
}