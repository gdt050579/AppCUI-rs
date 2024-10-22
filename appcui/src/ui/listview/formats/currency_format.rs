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
    USDWithSymbol,
    EUR,
    EURWithSymbol,
    GBP,
    GBPWithSymbol,
    YEN,
    YENWithSymbol,
    Bitcoin,
    BitcoinWithSymbol,
    RON,    
}

impl CurrencyFormat {
    pub(crate) const NUMERIC_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',').decimals(2);
    pub(crate) const fn name(&self)->(&'static str, usize){
        match self {
            CurrencyFormat::USD => ("USD", 3),
            CurrencyFormat::USDWithSymbol => ("$", 1),
            CurrencyFormat::EUR => ("EUR", 3),
            CurrencyFormat::EURWithSymbol => ("€", 1),
            CurrencyFormat::GBP => ("GBP", 3),
            CurrencyFormat::GBPWithSymbol => ("£", 1),
            CurrencyFormat::YEN => ("YEN", 3),
            CurrencyFormat::YENWithSymbol => ("¥", 1),
            CurrencyFormat::Bitcoin => ("BTC", 3),
            CurrencyFormat::BitcoinWithSymbol => ("₿", 1),
            CurrencyFormat::RON => ("RON", 3),            
        }
    }
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            CurrencyFormat::USD => &USD,
            CurrencyFormat::USDWithSymbol => &USD_SYMBOL,
            CurrencyFormat::EUR => &EUR,
            CurrencyFormat::EURWithSymbol => &EUR_SYMBOL,
            CurrencyFormat::GBP => &GBP,
            CurrencyFormat::GBPWithSymbol => &GBP_SYMBOL,
            CurrencyFormat::YEN => &YEN,
            CurrencyFormat::YENWithSymbol => &YEM_SYMBOL,
            CurrencyFormat::Bitcoin => &BTC,
            CurrencyFormat::BitcoinWithSymbol => &BTC_SYMBOL,
            CurrencyFormat::RON => &RON,
        }
    }
}

