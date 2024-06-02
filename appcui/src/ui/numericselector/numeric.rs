use super::Format;
use crate::utils::FormatNumber;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::fmt::Write;

#[allow(private_interfaces)]
pub(crate) trait Numeric: Add<Output = Self> + Sub<Output = Self> + Copy + Clone + PartialOrd + PartialEq + Display {
    fn write_to_string(&self, writer: &mut String, format: Format);
}

const DECIMAL_FORMAT: FormatNumber = FormatNumber::new(10);
const DIGIT_GROUPING_FORMAT: FormatNumber = FormatNumber::new(10).group(3, b',');

fn format_signed_number(value: i128, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => DECIMAL_FORMAT.write_signed(value, writer),
        Format::Percentage => { 
            DECIMAL_FORMAT.write_signed(value, writer);
            writer.push('%');
        },
        Format::DigitGrouping => DIGIT_GROUPING_FORMAT.write_signed(value, writer),
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
    }
}
fn format_float_number(value: f64, format: Format, writer: &mut String) {
    writer.clear();
    match format {
        Format::Decimal => write!(writer, "{}", value).unwrap(),
        Format::Percentage => todo!(),
        Format::DigitGrouping => todo!(),
    }
}

// default implementation for numeric types
impl Numeric for i8 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Numeric for i16 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Numeric for i32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Numeric for i64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Numeric for i128 {
    fn write_to_string(&self, writer: &mut String, format: Format) {  
        format_signed_number(*self, format, writer)
    }
}
impl Numeric for u8 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }
}
impl Numeric for u16 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Numeric for u32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Numeric for u64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }

}
impl Numeric for u128 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self, format, writer)
    }

}
impl Numeric for usize {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_unsigned_number(*self as u128, format, writer)
    }
}
impl Numeric for isize {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_signed_number(*self as i128, format, writer)
    }
}
impl Numeric for f32 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_float_number(*self as f64, format, writer)
    }
}
impl Numeric for f64 {
    fn write_to_string(&self, writer: &mut String, format: Format) {
        format_float_number(*self, format, writer)
    }
}
