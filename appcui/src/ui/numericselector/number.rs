use super::Format;
use crate::utils::FormatNumber;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::fmt::Write;
use std::str::FromStr;

pub trait Number: Add<Output = Self> + Sub<Output = Self> + Copy + Clone + PartialOrd + PartialEq + Display + FromStr {
    fn write_to_string(&self, writer: &mut String, format: Format);
}

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DIGIT_GROUPING_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');
const HEX_FORMAT: FormatNumber = FormatNumber::new(16);

fn format_signed_number(value: i128, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => DECIMAL_FORMAT.write_signed(value, writer),
        Format::Percentage => { 
            DECIMAL_FORMAT.write_signed(value, writer);
            writer.push('%');
        },
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
        },
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
        },
    }
}
fn format_float_number(value: f64, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => write!(writer, "{}", value).unwrap(),
        Format::Percentage => todo!(),
        Format::DigitGrouping => todo!(),
        Format::Hex => todo!(),
        Format::Size => todo!(),
    }
}

// default implementation for numeric types
impl Number for i8 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Number for i16 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Number for i32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Number for i64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Number for i128 {
    fn write_to_string(&self, writer: &mut String, format: Format) {  
        format_signed_number(*self, format, writer)
    }
}
impl Number for u8 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }
}
impl Number for u16 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Number for u32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Number for u64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Number for u128 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self, format, writer)
    }

}
impl Number for usize {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }
}
impl Number for isize {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Number for f32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_float_number(*self as f64, format, writer)
    }
}
impl Number for f64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_float_number(*self, format, writer)
    }
}
