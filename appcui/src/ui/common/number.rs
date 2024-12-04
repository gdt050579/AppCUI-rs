use crate::utils::FormatNumber;
use std::fmt::Display;
use std::ops::{Add, Sub, Div};
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Format {
    Decimal,
    Percentage,
    DigitGrouping,
    Hex,
    Size,
}

pub trait Number: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self>  + Copy + Clone + PartialOrd + PartialEq + Display + FromStr {
    const ONE: Self;
    fn write_to_string(&self, writer: &mut String, format: Format);
    fn is_zero(&self) -> bool;
    fn cast_to_u32(&self) -> u32;
}

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DIGIT_GROUPING_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');
const HEX_FORMAT: FormatNumber = FormatNumber::new(16);
const FLOAT_FORMAT: FormatNumber = FormatNumber::new(10).decimals(2);

fn format_signed_number(value: i128, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => DECIMAL_FORMAT.write_signed(value, writer),
        Format::Percentage => {
            DECIMAL_FORMAT.write_signed(value, writer);
            writer.push('%');
        }
        Format::DigitGrouping => DIGIT_GROUPING_FORMAT.write_signed(value, writer),
        Format::Hex => HEX_FORMAT.write_signed(value, writer),
        Format::Size => {
            if value < 1024 {
                DIGIT_GROUPING_FORMAT.write_signed(value, writer);
                writer.push_str(" B");
            } else if value < 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_signed(value / 1024, writer);
                writer.push_str(" KB");
            } else if value < 1024 * 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_signed(value / (1024 * 1024), writer);
                writer.push_str(" MB");
            } else if value < 1024 * 1024 * 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_signed(value / (1024 * 1024 * 1024), writer);
                writer.push_str(" GB");
            } else {
                DIGIT_GROUPING_FORMAT.write_signed(value / (1024 * 1024 * 1024 * 1024), writer);
                writer.push_str(" TB");
            }
        }
    }
}
fn format_unsigned_number(value: u128, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => DECIMAL_FORMAT.write_unsigned(value, writer),
        Format::Percentage => {
            DECIMAL_FORMAT.write_unsigned(value, writer);
            writer.push('%');
        }
        Format::DigitGrouping => DIGIT_GROUPING_FORMAT.write_unsigned(value, writer),
        Format::Hex => HEX_FORMAT.write_unsigned(value, writer),
        Format::Size => {
            if value < 1024 {
                DIGIT_GROUPING_FORMAT.write_unsigned(value, writer);
                writer.push_str(" B");
            } else if value < 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_unsigned(value / 1024, writer);
                writer.push_str(" KB");
            } else if value < 1024 * 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_unsigned(value / (1024 * 1024), writer);
                writer.push_str(" MB");
            } else if value < 1024 * 1024 * 1024 * 1024 {
                DIGIT_GROUPING_FORMAT.write_unsigned(value / (1024 * 1024 * 1024), writer);
                writer.push_str(" GB");
            } else {
                DIGIT_GROUPING_FORMAT.write_unsigned(value / (1024 * 1024 * 1024 * 1024), writer);
                writer.push_str(" TB");
            }
        }
    }
}
fn format_float_number(value: f64, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => FLOAT_FORMAT.write_float(value, writer),
        Format::Percentage => {
            FLOAT_FORMAT.write_float(value * 100.0f64, writer);
            writer.push('%');
        }
        Format::DigitGrouping => FLOAT_FORMAT.write_float(value, writer),
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
        }
    };
}

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

// // default implementation for numeric types
// impl Number for i8 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }
    
//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for i16 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for i32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for i64 {
//     const ONE:i64 = 1;
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for i128 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for u8 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for u16 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for u32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self;
//     }
// }
// impl Number for u64 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for u128 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for usize {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_unsigned_number(*self as u128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for isize {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_signed_number(*self as i128, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for f32 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_float_number(*self as f64, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0.0f32;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
// impl Number for f64 {
//     fn write_to_string(&self, writer: &mut String, format: Format) {
//         format_float_number(*self, format, writer)
//     }

//     fn is_zero(&self) -> bool {
//         return *self == 0.0f64;
//     }

//     fn cast_to_u32(&self) -> u32 {
//         return *self as u32;
//     }
// }
