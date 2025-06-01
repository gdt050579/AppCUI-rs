use crate::utils::FormatNumber;

const MMP: FormatNumber = FormatNumber::new(10).suffix(" mm²").group(3, b',');
const CMP: FormatNumber = FormatNumber::new(10).suffix(" cm²").group(3, b',');
const MP: FormatNumber = FormatNumber::new(10).suffix(" m² ").group(3, b',');
const KMP: FormatNumber = FormatNumber::new(10).suffix(" km²").group(3, b',');
const HECTARES: FormatNumber = FormatNumber::new(10).suffix(" ha ").group(3, b',');
const ARES: FormatNumber = FormatNumber::new(10).suffix(" a  ").group(3, b',');
const FTP: FormatNumber = FormatNumber::new(10).suffix(" ft²").group(3, b',');
const INP: FormatNumber = FormatNumber::new(10).suffix(" in²").group(3, b',');
const YDP: FormatNumber = FormatNumber::new(10).suffix(" yd²").group(3, b',');
const MIP: FormatNumber = FormatNumber::new(10).suffix(" mi²").group(3, b',');

// m2, cm2, km2, ha, a, ft2, in2, yd2, mi2
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AreaFormat {
    SquaredMillimeters,
    SquaredCentimeters,
    SquaredMeters,
    SquaredKilometers,
    Hectares,
    Ares,
    SquareFeet,
    SquareInches,
    SquareYards,
    SquareMiles,
}

impl AreaFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            AreaFormat::SquaredMillimeters => MMP.write_number(value, output),
            AreaFormat::SquaredCentimeters => CMP.write_number(value, output),
            AreaFormat::SquaredMeters => MP.write_number(value, output),
            AreaFormat::SquaredKilometers => KMP.write_number(value, output),
            AreaFormat::Hectares => HECTARES.write_number(value, output),
            AreaFormat::Ares => ARES.write_number(value, output),
            AreaFormat::SquareFeet => FTP.write_number(value, output),
            AreaFormat::SquareInches => INP.write_number(value, output),
            AreaFormat::SquareYards => YDP.write_number(value, output),
            AreaFormat::SquareMiles => MIP.write_number(value, output),
        }
    }
}
