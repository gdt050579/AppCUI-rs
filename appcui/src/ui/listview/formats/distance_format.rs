use crate::utils::FormatNumber;

const KM: FormatNumber = FormatNumber::new(10).suffix(" km").group(3, b',');
const M:  FormatNumber = FormatNumber::new(10).suffix(" m ").group(3, b',');
const CM: FormatNumber = FormatNumber::new(10).suffix(" cm").group(3, b',');
const MM: FormatNumber = FormatNumber::new(10).suffix(" mm").group(3, b',');
const IN: FormatNumber = FormatNumber::new(10).suffix(" in").group(3, b',');
const FT: FormatNumber = FormatNumber::new(10).suffix(" ft").group(3, b',');
const YD: FormatNumber = FormatNumber::new(10).suffix(" yd").group(3, b',');
const MI: FormatNumber = FormatNumber::new(10).suffix(" mi").group(3, b',');

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DistanceFormat {
    Kilometers,
    Meters,
    Centimeters,
    Millimeters,
    Inches,
    Feet,
    Yards,
    Miles,
}

impl DistanceFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            DistanceFormat::Kilometers => KM.write_number(value, output),
            DistanceFormat::Meters => M.write_number(value, output),
            DistanceFormat::Centimeters => CM.write_number(value, output),
            DistanceFormat::Millimeters => MM.write_number(value, output),
            DistanceFormat::Inches => IN.write_number(value, output),
            DistanceFormat::Feet => FT.write_number(value, output),
            DistanceFormat::Yards => YD.write_number(value, output),
            DistanceFormat::Miles => MI.write_number(value, output),
        }
    }
}
