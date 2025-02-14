use crate::utils::FormatNumber;

const BYTES: FormatNumber = FormatNumber::new(10).suffix(" B ").group(3, b',');
const KILO_BYTES: FormatNumber = FormatNumber::new(10).suffix(" KB").group(3, b',');
const MEGA_BYTES: FormatNumber = FormatNumber::new(10).suffix(" MB").group(3, b',');
const GIGA_BYTES: FormatNumber = FormatNumber::new(10).suffix(" GB").group(3, b',');
const TERA_BYTES: FormatNumber = FormatNumber::new(10).suffix(" TB").group(3, b',');
const BYTES_DECIMALS: FormatNumber = FormatNumber::new(10).suffix(" B ").group(3, b',').decimals(2);
const KILO_BYTES_DECIMALS: FormatNumber = FormatNumber::new(10).suffix(" KB").group(3, b',').decimals(2);
const MEGA_BYTES_DECIMALS: FormatNumber = FormatNumber::new(10).suffix(" MB").group(3, b',').decimals(2);
const GIGA_BYTES_DECIMALS: FormatNumber = FormatNumber::new(10).suffix(" GB").group(3, b',').decimals(2);
const TERA_BYTES_DECIMALS: FormatNumber = FormatNumber::new(10).suffix(" TB").group(3, b',').decimals(2);


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SizeFormat {
    Bytes,
    KiloBytes,
    MegaBytes,
    GigaBytes,
    TeraBytes,
    KiloBytesWithDecimals,
    MegaBytesWithDecimals,
    GigaBytesWithDecimals,
    TeraBytesWithDecimals,
    Auto,
    AutoWithDecimals,
}

impl SizeFormat {
    pub(crate) fn write<'a>(&self, value: u64, output: &'a mut [u8]) -> Option<&'a str> {
        match self {
            SizeFormat::Bytes => BYTES.write_number(value, output),
            SizeFormat::KiloBytes => KILO_BYTES.write_number(value / 1_000u64, output),
            SizeFormat::MegaBytes => MEGA_BYTES.write_number(value / 1_000_000u64, output),
            SizeFormat::GigaBytes => GIGA_BYTES.write_number(value / 1_000_000_000u64, output),
            SizeFormat::TeraBytes => TERA_BYTES.write_number(value / 1_000_000_000_000u64, output),
            SizeFormat::KiloBytesWithDecimals => KILO_BYTES_DECIMALS.write_fraction(value, 1_000u64, output),
            SizeFormat::MegaBytesWithDecimals => MEGA_BYTES_DECIMALS.write_fraction(value, 1_000_000u64, output),
            SizeFormat::GigaBytesWithDecimals => GIGA_BYTES_DECIMALS.write_fraction(value, 1_000_000_000u64, output),
            SizeFormat::TeraBytesWithDecimals => TERA_BYTES_DECIMALS.write_fraction(value, 1_000_000_000_000u64, output),
            SizeFormat::Auto => {
                if value<1_000u64 {
                    BYTES.write_number(value, output)
                } else if value<1_000_000u64 {
                    KILO_BYTES.write_number(value / 1_000u64, output)
                } else if value<1_000_000_000u64 {
                    MEGA_BYTES.write_number(value / 1_000_000u64, output)
                } else if value<1_000_000_000_000u64 {
                    GIGA_BYTES.write_number(value / 1_000_000_000u64, output)
                } else {
                    TERA_BYTES.write_number(value / 1_000_000_000_000u64, output)
                }
            },
            SizeFormat::AutoWithDecimals => {
                if value<1_000u64 {
                    BYTES_DECIMALS.write_fraction(value, 1u64, output)
                } else if value<1_000_000u64 {
                    KILO_BYTES_DECIMALS.write_fraction(value, 1_000u64, output)
                } else if value<1_000_000_000u64 {
                    MEGA_BYTES_DECIMALS.write_fraction(value, 1_000_000u64, output)
                } else if value<1_000_000_000_000u64 {
                    GIGA_BYTES_DECIMALS.write_fraction(value, 1_000_000_000u64, output)
                } else {
                    TERA_BYTES_DECIMALS.write_fraction(value, 1_000_000_000_000u64, output)
                }
            },
        }
    }
}
