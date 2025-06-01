use crate::utils::FormatNumber;

const CELSIUS: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°C");
const FAHRENHEIT: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°F");
const KELVIN: FormatNumber = FormatNumber::new(10).decimals(1).suffix("°K");

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TemperatureFormat {
    Celsius,
    Fahrenheit,
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
