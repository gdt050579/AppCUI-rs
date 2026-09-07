use crate::utils::FormatNumber;

const NORMAL: FormatNumber = FormatNumber::new(10).decimals(2);
const TWO_DIGITS: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2);
const THREE_DIGITS: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(3);
const FOUR_DIGITS: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(4);


#[derive(Copy, Clone, Eq, PartialEq)]
/// Precision and grouping used when rendering a floating-point list-item value.
///
/// `Normal` uses two decimal places without grouping; the other variants add
/// thousands separators and extra fractional digits.
pub enum FloatFormat {
    Normal,
    TwoDigits,
    ThreeDigits,
    FourDigits,
}

impl FloatFormat {
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            FloatFormat::Normal => &NORMAL,
            FloatFormat::TwoDigits => &TWO_DIGITS,
            FloatFormat::ThreeDigits => &THREE_DIGITS,
            FloatFormat::FourDigits => &FOUR_DIGITS,
        }
    }
}

