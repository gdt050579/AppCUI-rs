use crate::utils::FormatNumber;

const PERCENTAGE_FORMAT: FormatNumber = FormatNumber::new(10).suffix("%");
const PERCENTAGE_FORMAT_DIGITS: FormatNumber = FormatNumber::new(10).decimals(2).suffix("%");


#[derive(Copy, Clone, Eq, PartialEq)]
/// How a percentage list-item value is rendered.
///
/// `Normal` shows a whole-number percent; `Decimals` keeps two fractional digits.
pub enum PercentageFormat {
    /// Whole-number percent, for example `75%`.
    Normal,
    /// Two decimal places, for example `75.50%`.
    Decimals,
}

impl PercentageFormat {
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            PercentageFormat::Normal => &PERCENTAGE_FORMAT,
            PercentageFormat::Decimals => &PERCENTAGE_FORMAT_DIGITS,
        }
    }
}

