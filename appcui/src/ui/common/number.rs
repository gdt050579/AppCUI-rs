use crate::ui::common::NumberFormat;
use crate::utils::FormatNumber;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::str::FromStr;


pub trait Number: Add<Output = Self> + Sub<Output = Self> + Copy + Clone + PartialOrd + PartialEq + Display + FromStr {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>);
    fn to_f64(&self) -> f64;
    fn from_f64(v: f64) -> Self;
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

fn format_signed_number(value: i128, format: NumberFormat, writer: &mut String) {
    let mut output: [u8; 128] = [0; 128];
    writer.clear();
    let res = match format {
        NumberFormat::Decimal => DECIMAL_FORMAT.write_number(value, &mut output),
        NumberFormat::Percentage => DECIMAL_FORMAT_PERCENTAGE.write_number(value, &mut output),
        NumberFormat::DigitGrouping => DIGIT_GROUPING_FORMAT.write_number(value, &mut output),
        NumberFormat::Hex => HEX_FORMAT.write_number(value, &mut output),
        NumberFormat::Size => {
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
fn format_unsigned_number(value: u128, format: NumberFormat, writer: &mut String) {
    let mut output: [u8; 128] = [0; 128];
    writer.clear();
    let res = match format {
        NumberFormat::Decimal => DECIMAL_FORMAT.write_number(value, &mut output),
        NumberFormat::Percentage => DECIMAL_FORMAT_PERCENTAGE.write_number(value, &mut output),
        NumberFormat::DigitGrouping => DIGIT_GROUPING_FORMAT.write_number(value, &mut output),
        NumberFormat::Hex => HEX_FORMAT.write_number(value, &mut output),
        NumberFormat::Size => {
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
fn format_float_number(value: f64, format: NumberFormat, writer: &mut String) {
    let mut output: [u8; 32] = [0; 32];
    writer.clear();
    match format {
        NumberFormat::Decimal | NumberFormat::DigitGrouping => {
            if let Some(txt) = FLOAT_FORMAT.write_float(value, &mut output) {
                writer.push_str(txt);
            }
        }
        NumberFormat::Percentage => {
            if let Some(txt) = FLOAT_FORMAT.write_float(value * 100.0f64, &mut output) {
                writer.push_str(txt);
            }
        }
        NumberFormat::Hex => {
            let v = value as i128;
            format_signed_number(v, NumberFormat::Hex, writer);
        }
        NumberFormat::Size => {
            let v = value as i128;
            format_signed_number(v, NumberFormat::Size, writer);
        }
    }
}

// default implementation for numeric types
impl Number for i8 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self as i128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as i8
    }
}
impl Number for i16 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self as i128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as i16
    }
}
impl Number for i32 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self as i128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as i32
    }
}
impl Number for i64 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self as i128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as i64
    }
}
impl Number for i128 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as i128
    }
}
impl Number for u8 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self as u128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as u8
    }
}
impl Number for u16 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self as u128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as u16
    }
}
impl Number for u32 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self as u128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as u32
    }
}
impl Number for u64 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self as u128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as u64
    }
}
impl Number for u128 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as u128
    }
}
impl Number for usize {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_unsigned_number(*self as u128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as usize
    }
}
impl Number for isize {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_signed_number(*self as i128, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as isize
    }
}
impl Number for f32 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_float_number(*self as f64, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as f32
    }
}
impl Number for f64 {
    fn write_to_string(&self, writer: &mut String, format: impl Into<NumberFormat>) {
        format_float_number(*self, format.into(), writer)
    }
    fn to_f64(&self) -> f64 {
        *self
    }
    fn from_f64(v: f64) -> Self {
        v
    }
}
