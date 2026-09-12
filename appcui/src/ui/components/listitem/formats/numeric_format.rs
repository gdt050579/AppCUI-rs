use crate::utils::FormatNumber;

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DECIMAL_FORMAT_SEPARATIR: FormatNumber = FormatNumber::new(10).group(3, b',');
const HEX_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x");
const HEX_16_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(4);
const HEX_32_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(8);
const HEX_64_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(16);


#[derive(Copy, Clone, Eq, PartialEq)]
/// How an integer list-item value is rendered.
///
/// Choose plain decimal, thousands separators, or hexadecimal with an optional
/// fixed width (16, 32, or 64 bits).
pub enum NumericFormat {
    /// Plain decimal, for example `1234`.
    Normal,
    /// Thousands separators, for example `1,234`.
    Separator,
    /// Hexadecimal with `0x`, for example `0x4D2`.
    Hex,
    /// Hex padded to 16 bits, for example `0x04D2`.
    Hex16,
    /// Hex padded to 32 bits, for example `0x000004D2`.
    Hex32,
    /// Hex padded to 64 bits, for example `0x00000000000004D2`.
    Hex64,
}

impl NumericFormat {
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            NumericFormat::Normal => &DECIMAL_FORMAT,
            NumericFormat::Separator => &DECIMAL_FORMAT_SEPARATIR,
            NumericFormat::Hex => &HEX_FORMAT,
            NumericFormat::Hex16 => &HEX_16_FORMAT,
            NumericFormat::Hex32 => &HEX_32_FORMAT,
            NumericFormat::Hex64 => &HEX_64_FORMAT,
        }
    }
}

