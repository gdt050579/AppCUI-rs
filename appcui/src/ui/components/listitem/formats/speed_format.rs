use crate::utils::FormatNumber;

const KMH: FormatNumber = FormatNumber::new(10).suffix(" km/h").group(3, b',');
const METER_PER_HOUR: FormatNumber = FormatNumber::new(10).suffix("  m/h").group(3, b',');
const KMS: FormatNumber = FormatNumber::new(10).suffix(" km/s").group(3, b',');
const MPS: FormatNumber = FormatNumber::new(10).suffix("  m/s").group(3, b',');
const MPH: FormatNumber = FormatNumber::new(10).suffix("  mph").group(3, b',');
const MPS2: FormatNumber = FormatNumber::new(10).suffix("  mps").group(3, b',');
const KNOTS: FormatNumber = FormatNumber::new(10).suffix(" knots").group(3, b',');
const FPS: FormatNumber = FormatNumber::new(10).suffix(" ft/s").group(3, b',');
const MACH: FormatNumber = FormatNumber::new(10).suffix(" mach").group(3, b',');

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SpeedFormat {
    KilometersPerHour,
    MetersPerHour,
    KilometersPerSecond,
    MetersPerSecond,
    MilesPerHour,
    MilesPerSecond,
    Knots,
    FeetPerSecond,
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
