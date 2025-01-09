use crate::utils::FormatNumber;

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DECIMAL_FORMAT_SEPARATIR: FormatNumber = FormatNumber::new(10).group(3, b',');
const HEX_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x");
const HEX_16_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(4);
const HEX_32_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(8);
const HEX_64_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x").representation_digits(16);


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NumericFormat {
    Normal,
    Separator,
    Hex,
    Hex16,
    Hex32,
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

