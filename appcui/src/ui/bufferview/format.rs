mod hex;

use super::OutputBuffer;

#[derive(Clone, Copy)]
pub enum Columns {
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
pub enum Format {
    Hex(BytesCount),
}
impl Format {
    pub(super) fn write(&self, bytes: [u8;8], output: &mut OutputBuffer) {
        match self {
            Format::Hex(bytes_count) => hex::write(bytes, *bytes_count, output),
        }
    }
    #[inline(always)]
    pub(super) fn bytes_count(&self) -> u8 {
        match self {
            Format::Hex(bytes_count) => *bytes_count as u8,
        }
    }
}

pub(super) struct Representation {
    pub(super) format: Format,
    pub(super) endian: Endian,
    pub(super) columns: Columns,
}
impl Representation {
    pub(super) fn new() -> Self {
        Self {
            format: Format::Hex(BytesCount::One),
            endian: Endian::Little,
            columns: Columns::Fixed(8),
        }
    }
}