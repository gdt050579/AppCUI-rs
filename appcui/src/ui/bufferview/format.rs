mod hex;
mod oct;
mod bin;

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
#[derive(Clone, Copy)]
pub enum Endian {
    Little,
    Big,
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BytesCount {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
}

#[derive(Clone, Copy)]
pub enum DataRepresentationFormat {
    Hex(BytesCount),
    Oct,
    Bin,
    Char,
}
impl DataRepresentationFormat {
    pub(super) fn write(&self, bytes: [u8;8], output: &mut OutputBuffer) {
        match self {
            DataRepresentationFormat::Hex(bytes_count) => hex::write(bytes, *bytes_count, output),
            DataRepresentationFormat::Oct => oct::write(bytes, output),
            DataRepresentationFormat::Bin => bin::write(bytes, output),
            DataRepresentationFormat::Char => { output.set_len(1); output.set(0, bytes[0]); },
        }
    }
    #[inline(always)]
    pub(super) fn bytes_count(&self) -> u8 {
        match self {
            DataRepresentationFormat::Hex(bytes_count) => *bytes_count as u8,
            DataRepresentationFormat::Oct => 1,
            DataRepresentationFormat::Bin => 1,
            DataRepresentationFormat::Char => 1,
        }
    }
    #[inline(always)]
    pub(super) fn display_chars(&self) -> u32 {
        match self {
            DataRepresentationFormat::Hex(byte_count) => (*byte_count as u32) * 2,
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
            format: DataRepresentationFormat::Hex(BytesCount::One),
            endian: Endian::Little,
            columns: ColumnsCount::Fixed(8),
            offset_format: OffsetFormat::Hex,
            columns_count: 8,
            rows_count: 1,
        }
    }
}