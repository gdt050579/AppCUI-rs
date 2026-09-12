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
/// Unit used when rendering an area value in a list item.
///
/// Selects metric or imperial squared units (mm², m², ha, ft², and similar).
pub enum AreaFormat {
    /// Square millimeters, for example `12 mm²`.
    SquaredMillimeters,
    /// Square centimeters, for example `12 cm²`.
    SquaredCentimeters,
    /// Square meters, for example `12 m²`.
    SquaredMeters,
    /// Square kilometers, for example `12 km²`.
    SquaredKilometers,
    /// Hectares, for example `12 ha`.
    Hectares,
    /// Ares, for example `12 a`.
    Ares,
    /// Square feet, for example `12 ft²`.
    SquareFeet,
    /// Square inches, for example `12 in²`.
    SquareInches,
    /// Square yards, for example `12 yd²`.
    SquareYards,
    /// Square miles, for example `12 mi²`.
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
