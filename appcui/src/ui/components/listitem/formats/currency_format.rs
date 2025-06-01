use crate::utils::FormatNumber;

const USD: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("USD ");
const USD_SYMBOL: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("$ ");
const EUR: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("EUR ");
const EUR_SYMBOL: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("€ ");
const GBP: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("GBP ");
const GBP_SYMBOL: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("£ ");
const YEN: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("YEN ");
const YEM_SYMBOL: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("¥ ");
const BTC: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("BTC ");
const BTC_SYMBOL: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("₿ ");
const RON: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2).prefix("RON ");

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CurrencyFormat {
    USD,
    USDSymbol,
    EUR,
    EURSymbol,
    GBP,
    GBPSymbol,
    YEN,
    YENSymbol,
    Bitcoin,
    BitcoinSymbol,
    RON,
}

impl CurrencyFormat {
    pub(crate) const NUMERIC_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2);
    pub(crate) const fn name(&self) -> (&'static str, usize) {
        match self {
            CurrencyFormat::USD => ("USD", 3),
            CurrencyFormat::USDSymbol => ("$", 1),
            CurrencyFormat::EUR => ("EUR", 3),
            CurrencyFormat::EURSymbol => ("€", 1),
            CurrencyFormat::GBP => ("GBP", 3),
            CurrencyFormat::GBPSymbol => ("£", 1),
            CurrencyFormat::YEN => ("YEN", 3),
            CurrencyFormat::YENSymbol => ("¥", 1),
            CurrencyFormat::Bitcoin => ("BTC", 3),
            CurrencyFormat::BitcoinSymbol => ("₿", 1),
            CurrencyFormat::RON => ("RON", 3),
        }
    }
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            CurrencyFormat::USD => &USD,
            CurrencyFormat::USDSymbol => &USD_SYMBOL,
            CurrencyFormat::EUR => &EUR,
            CurrencyFormat::EURSymbol => &EUR_SYMBOL,
            CurrencyFormat::GBP => &GBP,
            CurrencyFormat::GBPSymbol => &GBP_SYMBOL,
            CurrencyFormat::YEN => &YEN,
            CurrencyFormat::YENSymbol => &YEM_SYMBOL,
            CurrencyFormat::Bitcoin => &BTC,
            CurrencyFormat::BitcoinSymbol => &BTC_SYMBOL,
            CurrencyFormat::RON => &RON,
        }
    }
}
