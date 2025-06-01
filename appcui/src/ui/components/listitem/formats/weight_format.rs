use crate::utils::FormatNumber;

const KG: FormatNumber = FormatNumber::new(10).suffix(" kg").group(3, b',');
const GR: FormatNumber = FormatNumber::new(10).suffix(" gr").group(3, b',');
const MG: FormatNumber = FormatNumber::new(10).suffix(" mg").group(3, b',');
const LB: FormatNumber = FormatNumber::new(10).suffix(" lb").group(3, b',');
const TON: FormatNumber = FormatNumber::new(10).suffix(" t ").group(3, b',');

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WeightFormat {
    Grams,
    Milligrams,
    Kilograms,
    Pounds,
    Tons,
}

impl WeightFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            WeightFormat::Kilograms => KG.write_number(value, output),
            WeightFormat::Grams => GR.write_number(value, output),
            WeightFormat::Milligrams => MG.write_number(value, output),
            WeightFormat::Pounds => LB.write_number(value, output),
            WeightFormat::Tons => TON.write_number(value, output),
        }
    }
}
