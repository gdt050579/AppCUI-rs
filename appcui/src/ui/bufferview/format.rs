mod hex;
mod oct;
mod bin;
mod uint;
mod int;
mod float;

#[cfg(test)]
mod tests;

use super::OutputBuffer;

#[derive(Copy, Clone)]
pub enum OffsetFormat {
    Hex,
    Dec
}
#[derive(Clone, Copy)]
pub enum ColumnsCount {
    Fixed(u8),
    Auto,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum HexFormat {
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum UIntFormat {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum IntFormat {
    I8 = 1,
    I16 = 2,
    I32 = 4,
    I64 = 8,
}
#[derive(Clone, Copy)]
pub enum FloatFormat {
    Scientific32,
    Scientific64,
    E4M3,
    E5M2,
}

#[derive(Clone, Copy)]
pub enum DataRepresentationFormat {
    Hex(HexFormat),
    Oct,
    Bin,
    UInt(UIntFormat),
    Int(IntFormat),
    Float(FloatFormat),
    Char,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ValidateResult {
    Valid,
    FormatError,
    Update
}
impl DataRepresentationFormat {
    pub(super) fn write(&self, bytes: [u8;8], output: &mut OutputBuffer) {
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
    pub(super) fn bytes_count(&self) -> u8 {
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
    pub(super) fn display_chars(&self) -> u32 {
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
    pub(super) fn is_char(&self) -> bool {
        match self {
            DataRepresentationFormat::Char => true,
            _ => false,
        }
    }
    #[inline(always)]
    pub(super) fn validate(&self, text: &str) -> ValidateResult {
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
    pub(super) fn convert_to_bytes(&self, text: &str) -> ([u8; 8], u8) {
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