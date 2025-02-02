use crate::utils::FormatNumber;

const PERCENTAGE_FORMAT: FormatNumber = FormatNumber::new(10).suffix("%");
const PERCENTAGE_FORMAT_DIGITS: FormatNumber = FormatNumber::new(10).decimals(2).suffix("%");


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PercentageFormat {
    Normal,
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

