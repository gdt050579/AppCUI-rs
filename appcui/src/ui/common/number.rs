use crate::utils::FormatNumber;
use std::fmt::Display;
use std::ops::{Add, Sub, Div, Mul};
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Format {
    Decimal,
    Percentage,
    DigitGrouping,
    Hex,
    Size,
}

pub trait Number: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Copy + Clone + PartialOrd + PartialEq + Display + FromStr{
    const ONE: Self;
    fn write_to_string(&self, writer: &mut String, format: Format);
    fn is_zero(&self) -> bool;
    fn cast_to_u32(&self) -> u32;
    fn abs(&self) -> Self;

    fn cast_signed_number(value: i128) -> Self;
    fn cast_unsigned_number(value: u128) -> Self;
    fn cast_float_number(value: f64) -> Self;
}

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DECIMAL_FORMAT_PERCENTAGE: FormatNumber = FormatNumber::new(10).suffix("%");
const DIGIT_GROUPING_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');
const SIZE_B: FormatNumber = FormatNumber::new(10).group(3, b',').suffix(" B");
const SIZE_KB: FormatNumber = FormatNumber::new(10).group(3, b',').suffix(" KB");
const SIZE_MB: FormatNumber = FormatNumber::new(10).group(3, b',').suffix(" MB");
const SIZE_GB: FormatNumber = FormatNumber::new(10).group(3, b',').suffix(" GB");
const SIZE_TB: FormatNumber = FormatNumber::new(10).group(3, b',').suffix(" TB");
const HEX_FORMAT: FormatNumber = FormatNumber::new(16).prefix("0x");
const FLOAT_FORMAT: FormatNumber = FormatNumber::new(10).decimals(2);

fn format_signed_number(value: i128, format: Format, writer: &mut String) {
    let mut output: [u8; 128] = [0; 128];
    writer.clear();
    let res = match format {
        Format::Decimal => DECIMAL_FORMAT.write_number(value, &mut output),
        Format::Percentage => DECIMAL_FORMAT_PERCENTAGE.write_number(value, &mut output),
        Format::DigitGrouping => DIGIT_GROUPING_FORMAT.write_number(value, &mut output),
        Format::Hex => HEX_FORMAT.write_number(value, &mut output),
        Format::Size => {
            if value < 1024 {
                SIZE_B.write_number(value, &mut output)
            } else if value < 1024 * 1024 {
                SIZE_KB.write_number(value / 1024, &mut output)
            } else if value < 1024 * 1024 * 1024 {
                SIZE_MB.write_number(value / (1024 * 1024), &mut output)
            } else if value < 1024 * 1024 * 1024 * 1024 {
                SIZE_GB.write_number(value / (1024 * 1024 * 1024), &mut output)
            } else {
                SIZE_TB.write_number(value / (1024 * 1024 * 1024 * 1024), &mut output)
            }
        }
    };
    if let Some(txt) = res {
        writer.push_str(txt);
    }
}
fn format_unsigned_number(value: u128, format: Format, writer: &mut String) {
    let mut output: [u8; 128] = [0; 128];
    writer.clear();
    let res = match format {
        Format::Decimal => DECIMAL_FORMAT.write_number(value, &mut output),
        Format::Percentage => DECIMAL_FORMAT_PERCENTAGE.write_number(value, &mut output),
        Format::DigitGrouping => DIGIT_GROUPING_FORMAT.write_number(value, &mut output),
        Format::Hex => HEX_FORMAT.write_number(value, &mut output),
        Format::Size => {
            if value < 1024 {
                SIZE_B.write_number(value, &mut output)
            } else if value < 1024 * 1024 {
                SIZE_KB.write_number(value / 1024, &mut output)
            } else if value < 1024 * 1024 * 1024 {
                SIZE_MB.write_number(value / (1024 * 1024), &mut output)
            } else if value < 1024 * 1024 * 1024 * 1024 {
                SIZE_GB.write_number(value / (1024 * 1024 * 1024), &mut output)
            } else {
                SIZE_TB.write_number(value / (1024 * 1024 * 1024 * 1024), &mut output)
            }
        }
    };
    if let Some(txt) = res {
        writer.push_str(txt);
    }
}
fn format_float_number(value: f64, format: Format, writer: &mut String) {
    let mut output: [u8; 32] = [0; 32];
    writer.clear();
    match format {
        Format::Decimal | Format::DigitGrouping => {
            if let Some(txt) = FLOAT_FORMAT.write_float(value, &mut output) {
                writer.push_str(txt);
            }
        }
        Format::Percentage => {
            if let Some(txt) = FLOAT_FORMAT.write_float(value * 100.0f64, &mut output) {
                writer.push_str(txt);
            }
        }
        Format::Hex => {
            let v = value as i128;
            format_signed_number(v, Format::Hex, writer);
        }
        Format::Size => {
            let v = value as i128;
            format_signed_number(v, Format::Size, writer);
        }
    }
}

macro_rules! IMPLEMENT_FOR_SIGNED {
    ($t: ty) => {
        impl Number for $t {
            const ONE: $t = 1;
            fn write_to_string(&self, writer: &mut String, format: Format) {
                format_signed_number(*self as i128, format, writer)
            }
        
            fn is_zero(&self) -> bool {
                return *self == 0;
            }
            
            fn cast_to_u32(&self) -> u32 {
                return *self as u32;
            }

            fn abs(&self) -> Self {
                return (*self).abs();
            }

            fn cast_signed_number(value: i128) -> Self {
                return value as Self;
            }
            fn cast_unsigned_number(value: u128) -> Self {
                return value as Self;
            }
            fn cast_float_number(value: f64) -> Self {
                return value as Self;
            }
        }
    };
}

macro_rules! IMPLEMENT_FOR_UNSIGNED {
    ($t: ty) => {
        impl Number for $t {
            const ONE: $t = 1;
            fn write_to_string(&self, writer: &mut String, format: Format) {
                format_unsigned_number(*self as u128, format, writer)
            }
        
            fn is_zero(&self) -> bool {
                return *self == 0;
            }
            
            fn cast_to_u32(&self) -> u32 {
                return *self as u32;
            }

            fn abs(&self) -> Self {
                return *self;
            }

            fn cast_signed_number(value: i128) -> Self {
                return value as Self;
            }
            fn cast_unsigned_number(value: u128) -> Self {
                return value as Self;
            }
            fn cast_float_number(value: f64) -> Self {
                return value as Self;
            }
        }
    };
}

macro_rules! IMPLEMENT_FOR_FLOAT {
    ($t: ty) => {
        impl Number for $t {
            const ONE: $t = 1.0;
            fn write_to_string(&self, writer: &mut String, format: Format) {
                format_float_number(*self as f64, format, writer)
            }
        
            fn is_zero(&self) -> bool {
                return *self == 0.0;
            }
            
            fn cast_to_u32(&self) -> u32 {
                return *self as u32;
            }

            fn abs(&self) -> Self {
                return (*self).abs();
            }
            
            fn cast_signed_number(value: i128) -> Self {
                return value as Self;
            }
            fn cast_unsigned_number(value: u128) -> Self {
                return value as Self;
            }
            fn cast_float_number(value: f64) -> Self {
                return value as Self;
            }
        }
    };
}

// default implementation for numeric types
IMPLEMENT_FOR_SIGNED!(i8);
IMPLEMENT_FOR_SIGNED!(i16);
IMPLEMENT_FOR_SIGNED!(i32);
IMPLEMENT_FOR_SIGNED!(i64);
IMPLEMENT_FOR_SIGNED!(i128);
IMPLEMENT_FOR_UNSIGNED!(u8);
IMPLEMENT_FOR_UNSIGNED!(u16);
IMPLEMENT_FOR_UNSIGNED!(u32);
IMPLEMENT_FOR_UNSIGNED!(u64);
IMPLEMENT_FOR_UNSIGNED!(u128);
IMPLEMENT_FOR_FLOAT!(f32);
IMPLEMENT_FOR_FLOAT!(f64);

// impl Number for i8 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }
// }
// impl Number for i16 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }
// }
// impl Number for i32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }
// }
// impl Number for i64 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }
// }
// impl Number for i128 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self, format, writer)
//     }
// }
// impl Number for u8 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }
// }
// impl Number for u16 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }
// }
// impl Number for u32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }
// }
// impl Number for u64 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }
// }
// impl Number for u128 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self, format, writer)
//     }
// }
// impl Number for usize {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }
// }
// impl Number for isize {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }
// }
// impl Number for f32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_float_number(*self as f64, format, writer)
//     }
// }
// impl Number for f64 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_float_number(*self, format, writer)
//     }
// }
