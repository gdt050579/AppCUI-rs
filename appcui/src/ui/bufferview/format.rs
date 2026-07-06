//! Display and editing formats for the data panel of a [`super::BufferView`].
//!
//! These types control how raw bytes are rendered, grouped into columns, and interpreted
//! when the user edits values in place.

mod hex;
mod oct;
mod bin;
mod uint;
mod int;
mod float;

#[cfg(test)]
mod tests;

use super::OutputBuffer;

/// How byte offsets are shown in the address column of a [`super::BufferView`].
#[derive(Copy, Clone)]
pub enum OffsetFormat {
    /// Offsets are shown as hexadecimal values.
    Hex,
    /// Offsets are shown as decimal values.
    Dec,
}

/// How many data values are laid out on each row of the data panel.
#[derive(Clone, Copy)]
pub enum ColumnsCount {
    /// A fixed number of columns per row.
    Fixed(u8),
    /// As many columns as fit in the current control width.
    Auto,
}

/// Byte order used when reading and writing multi-byte numeric values.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Endian {
    /// Least-significant byte first.
    Little,
    /// Most-significant byte first.
    Big,
}

/// Width of a single hex value in the data panel.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum HexFormat {
    /// One byte (`NN`).
    Byte = 1,
    /// Two bytes (`NNNN`).
    Word = 2,
    /// Four bytes (`NNNNNNNN`).
    DWord = 4,
    /// Eight bytes (`NNNNNNNNNNNNNNNN`).
    QWord = 8,
}

/// Width of an unsigned integer value in the data panel.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum UIntFormat {
    /// 8-bit unsigned integer.
    U8 = 1,
    /// 16-bit unsigned integer.
    U16 = 2,
    /// 32-bit unsigned integer.
    U32 = 4,
    /// 64-bit unsigned integer.
    U64 = 8,
}

/// Width of a signed integer value in the data panel.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum IntFormat {
    /// 8-bit signed integer.
    I8 = 1,
    /// 16-bit signed integer.
    I16 = 2,
    /// 32-bit signed integer.
    I32 = 4,
    /// 64-bit signed integer.
    I64 = 8,
}

/// Floating-point encoding shown in the data panel.
#[derive(Clone, Copy)]
pub enum FloatFormat {
    /// 32-bit IEEE 754 value in scientific notation.
    Scientific32,
    /// 64-bit IEEE 754 value in scientific notation.
    Scientific64,
    /// 8-bit OCP FP8 E4M3 value.
    E4M3,
    /// 8-bit OCP FP8 E5M2 value.
    E5M2,
}

/// Active data representation for the main panel of a [`super::BufferView`].
#[derive(Clone, Copy)]
pub enum DataRepresentationFormat {
    /// Hexadecimal values with the given width.
    Hex(HexFormat),
    /// Octal byte values.
    Oct,
    /// Binary byte values.
    Bin,
    /// Unsigned integers with the given width.
    UInt(UIntFormat),
    /// Signed integers with the given width.
    Int(IntFormat),
    /// Floating-point values with the given encoding.
    Float(FloatFormat),
    /// Raw characters (one byte per column).
    Char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ValidateResult {
    Valid,
    FormatError,
    Update
}
impl DataRepresentationFormat {
    pub(crate) fn write(&self, bytes: [u8; 8], output: &mut OutputBuffer) {
        match self {
            DataRepresentationFormat::Hex(format) => hex::write(bytes, *format, output),
            DataRepresentationFormat::Oct => oct::write(bytes, output),
            DataRepresentationFormat::Bin => bin::write(bytes, output),
            DataRepresentationFormat::Char => { output.set_len(1); output.set(0, bytes[0]); },
            DataRepresentationFormat::UInt(format) => uint::write(bytes, *format, output),
            DataRepresentationFormat::Int(format) => int::write(bytes, *format, output),
            DataRepresentationFormat::Float(format) => float::write(bytes, *format, output),
        }
    }
    #[inline(always)]
    pub(crate) fn bytes_count(&self) -> u8 {
        match self {
            DataRepresentationFormat::Hex(format) => *format as u8,
            DataRepresentationFormat::UInt(format) => *format as u8,
            DataRepresentationFormat::Int(format) => *format as u8,
            DataRepresentationFormat::Float(format) => float::bytes_count(*format),
            DataRepresentationFormat::Oct => 1,
            DataRepresentationFormat::Bin => 1,
            DataRepresentationFormat::Char => 1,
        }
    }
    #[inline(always)]
    pub(crate) fn display_chars(&self) -> u32 {
        match self {
            DataRepresentationFormat::Hex(format) => (*format as u32) * 2,
            DataRepresentationFormat::UInt(format) => uint::display_chars(*format),
            DataRepresentationFormat::Int(format) => int::display_chars(*format),
            DataRepresentationFormat::Float(format) => float::display_chars(*format),
            DataRepresentationFormat::Oct => 3,
            DataRepresentationFormat::Bin => 8,
            DataRepresentationFormat::Char => 1,
        }
    }
    #[inline(always)]
    pub(crate) fn is_char(&self) -> bool {
        match self {
            DataRepresentationFormat::Char => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(crate) fn validate(&self, text: &str) -> ValidateResult {
        if text.is_empty() {
            return ValidateResult::Valid;
        }
        match self {
            DataRepresentationFormat::Hex(format) => hex::validate(text, *format),
            DataRepresentationFormat::UInt(format) => uint::validate(text, *format),
            DataRepresentationFormat::Int(format) => int::validate(text, *format),
            DataRepresentationFormat::Oct => oct::validate(text),
            DataRepresentationFormat::Bin => bin::validate(text),
            DataRepresentationFormat::Char => ValidateResult::Update,
            DataRepresentationFormat::Float(format) => float::validate(text, *format),
        }
    }
    pub(crate) fn convert_to_bytes(&self, text: &str) -> ([u8; 8], u8) {
        if text.is_empty() {
            return ([0; 8], 0);
        }
        match self {
            DataRepresentationFormat::Hex(format) => hex::convert_to_bytes(text, *format),
            DataRepresentationFormat::UInt(format) => uint::convert_to_bytes(text, *format),
            DataRepresentationFormat::Int(format) => int::convert_to_bytes(text, *format),
            DataRepresentationFormat::Oct => oct::convert_to_bytes(text),
            DataRepresentationFormat::Bin => bin::convert_to_bytes(text),
            DataRepresentationFormat::Char => {
                let b = text.as_bytes();
                let mut output = [0; 8];
                let len = b.len().min(8);
                output[..len].copy_from_slice(&b[..len]);
                (output, len as u8)
            },
            DataRepresentationFormat::Float(format) => float::convert_to_bytes(text, *format),
        }
    }
}

pub(super) struct Representation {
    pub(super) format: DataRepresentationFormat,
    pub(super) endian: Endian,
    pub(super) columns: ColumnsCount,
    pub(super) columns_count: u32,
    pub(super) rows_count: u32,
    pub(super) offset_format: OffsetFormat,
}
impl Representation {
    pub(super) fn new() -> Self {
        Self {
            format: DataRepresentationFormat::Hex(HexFormat::Byte),
            endian: Endian::Little,
            columns: ColumnsCount::Fixed(8),
            offset_format: OffsetFormat::Hex,
            columns_count: 8,
            rows_count: 1,
        }
    }
}