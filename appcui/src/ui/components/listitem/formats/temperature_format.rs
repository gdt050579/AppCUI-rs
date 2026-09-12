use crate::utils::FormatNumber;

const CELSIUS: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°C");
const FAHRENHEIT: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°F");
const KELVIN: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°K");


#[derive(Copy, Clone, Eq, PartialEq)]
/// Temperature scale used when rendering a list-item value.
///
/// Choose Celsius, Fahrenheit, or Kelvin, each with a one-decimal display.
pub enum TemperatureFormat {
    /// Celsius, for example `21.5°C`.
    Celsius,
    /// Fahrenheit, for example `70.7°F`.
    Fahrenheit,
    /// Kelvin, for example `294.6°K`.
    Kelvin,
}

impl TemperatureFormat {
    pub(crate) const fn formatter(&self) -> &'static FormatNumber {
        match self {
            TemperatureFormat::Celsius => &CELSIUS,
            TemperatureFormat::Fahrenheit => &FAHRENHEIT,
            TemperatureFormat::Kelvin => &KELVIN,
        }
    }
}

