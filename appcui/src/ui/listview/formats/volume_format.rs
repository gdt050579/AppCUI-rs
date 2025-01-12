use crate::utils::FormatNumber;

const CUBIC_MM: FormatNumber = FormatNumber::new(10).suffix(" mm³").group(3, b',');
const CUBIC_CM: FormatNumber = FormatNumber::new(10).suffix(" cm³").group(3, b',');
const CUBIC_M: FormatNumber = FormatNumber::new(10).suffix(" m³ ").group(3, b',');
const CUBIC_KM: FormatNumber = FormatNumber::new(10).suffix(" km³").group(3, b',');
const LITERS: FormatNumber = FormatNumber::new(10).suffix(" l  ").group(3, b',');
const MILLILITERS: FormatNumber = FormatNumber::new(10).suffix(" ml ").group(3, b',');
const GALLONS: FormatNumber = FormatNumber::new(10).suffix(" gal").group(3, b',');
const CUBIC_FT: FormatNumber = FormatNumber::new(10).suffix(" ft³").group(3, b',');
const CUBIC_IN: FormatNumber = FormatNumber::new(10).suffix(" in³").group(3, b',');
const CUBIC_YD: FormatNumber = FormatNumber::new(10).suffix(" yd³").group(3, b',');
const CUBIC_MI: FormatNumber = FormatNumber::new(10).suffix(" mi³").group(3, b',');




#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VolumeFormat {
    CubicMilimeters,
    CubicCentimeters,
    CubicMeters,
    CubicKilometers,
    Liters,
    Milliliters,
    Gallons,
    CubicFeet,
    CubicInches,
    CubicYards,
    CubicMiles,
}

impl VolumeFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            VolumeFormat::CubicMilimeters => CUBIC_MM.write_number(value, output),
            VolumeFormat::CubicCentimeters => CUBIC_CM.write_number(value, output),
            VolumeFormat::CubicMeters => CUBIC_M.write_number(value, output),
            VolumeFormat::CubicKilometers => CUBIC_KM.write_number(value, output),
            VolumeFormat::Liters => LITERS.write_number(value, output),
            VolumeFormat::Milliliters => MILLILITERS.write_number(value, output),
            VolumeFormat::Gallons => GALLONS.write_number(value, output),
            VolumeFormat::CubicFeet => CUBIC_FT.write_number(value, output),
            VolumeFormat::CubicInches => CUBIC_IN.write_number(value, output),
            VolumeFormat::CubicYards => CUBIC_YD.write_number(value, output),
            VolumeFormat::CubicMiles => CUBIC_MI.write_number(value, output),
        }
    }
}
