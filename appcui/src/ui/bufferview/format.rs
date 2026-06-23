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
pub enum Format {
    Hex
}
impl Format {
    pub fn write(&self, bytes: [u8;8], output: &mut OutputBuffer) {
        match self {
            Format::Hex => hex::write(bytes, output),
        }
    }
}