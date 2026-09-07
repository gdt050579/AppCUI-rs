use crate::utils::FormatNumber;

const KMH: FormatNumber             = FormatNumber::new(10).suffix(" km/h").group(3, b',');
const METER_PER_HOUR: FormatNumber  = FormatNumber::new(10).suffix("  m/h").group(3, b',');
const KMS: FormatNumber             = FormatNumber::new(10).suffix(" km/s").group(3, b',');
const MPS: FormatNumber             = FormatNumber::new(10).suffix("  m/s").group(3, b',');
const MPH: FormatNumber             = FormatNumber::new(10).suffix("  mph").group(3, b',');
const MPS2: FormatNumber            = FormatNumber::new(10).suffix("  mps").group(3, b',');
const KNOTS: FormatNumber           = FormatNumber::new(10).suffix(" knots").group(3, b',');
const FPS: FormatNumber             = FormatNumber::new(10).suffix(" ft/s").group(3, b',');
const MACH: FormatNumber            = FormatNumber::new(10).suffix(" mach").group(3, b',');


#[derive(Copy, Clone, Eq, PartialEq)]
/// Unit used when rendering a speed list-item value.
///
/// Selects km/h, m/s, mph, knots, Mach, and similar units.
pub enum SpeedFormat {
    /// Kilometers per hour, for example `90 km/h`.
    KilometersPerHour,
    /// Meters per hour, for example `90 m/h`.
    MetersPerHour,
    /// Kilometers per second, for example `3 km/s`.
    KilometersPerSecond,
    /// Meters per second, for example `12 m/s`.
    MetersPerSecond,
    /// Miles per hour, for example `55 mph`.
    MilesPerHour,
    /// Miles per second, for example `1 mps`.
    MilesPerSecond,
    /// Nautical knots, for example `12 knots`.
    Knots,
    /// Feet per second, for example `10 ft/s`.
    FeetPerSecond,
    /// Mach number, for example `1 mach`.
    Mach,    
}

impl SpeedFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            SpeedFormat::KilometersPerHour => KMH.write_number(value, output),        
            SpeedFormat::MetersPerHour => METER_PER_HOUR.write_number(value, output),
            SpeedFormat::KilometersPerSecond => KMS.write_number(value, output),
            SpeedFormat::MetersPerSecond => MPS.write_number(value, output),
            SpeedFormat::MilesPerHour => MPH.write_number(value, output),
            SpeedFormat::MilesPerSecond => MPS2.write_number(value, output),
            SpeedFormat::Knots => KNOTS.write_number(value, output),
            SpeedFormat::FeetPerSecond => FPS.write_number(value, output),
            SpeedFormat::Mach => MACH.write_number(value, output),            
        }
    }
}
