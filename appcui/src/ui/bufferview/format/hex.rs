use std::ascii;

use super::super::BytesCount;
use super::super::OutputBuffer;
use super::ValidateResult;

static HEX_CHARS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub(super) fn write(bytes: [u8; 8], bytes_count: BytesCount, output: &mut OutputBuffer) {
    match bytes_count {
        BytesCount::One => {
            output.set(0, HEX_CHARS[(bytes[0] >> 4) as usize]);
            output.set(1, HEX_CHARS[(bytes[0] & 0x0F) as usize]);
            output.set_len(2);
        }
        BytesCount::Two => {
            output.set(0, HEX_CHARS[(bytes[1] >> 4) as usize]);
            output.set(1, HEX_CHARS[(bytes[1] & 0x0F) as usize]);
            output.set(2, HEX_CHARS[(bytes[0] >> 4) as usize]);
            output.set(3, HEX_CHARS[(bytes[0] & 0x0F) as usize]);
            output.set_len(4);
        }
        BytesCount::Four => {
            output.set(0, HEX_CHARS[(bytes[3] >> 4) as usize]);
            output.set(1, HEX_CHARS[(bytes[3] & 0x0F) as usize]);
            output.set(2, HEX_CHARS[(bytes[2] >> 4) as usize]);
            output.set(3, HEX_CHARS[(bytes[2] & 0x0F) as usize]);
            output.set(4, HEX_CHARS[(bytes[1] >> 4) as usize]);
            output.set(5, HEX_CHARS[(bytes[1] & 0x0F) as usize]);
            output.set(6, HEX_CHARS[(bytes[0] >> 4) as usize]);
            output.set(7, HEX_CHARS[(bytes[0] & 0x0F) as usize]);
            output.set_len(8);
        }
        BytesCount::Eight => {
            output.set(0, HEX_CHARS[(bytes[7] >> 4) as usize]);
            output.set(1, HEX_CHARS[(bytes[7] & 0x0F) as usize]);
            output.set(2, HEX_CHARS[(bytes[6] >> 4) as usize]);
            output.set(3, HEX_CHARS[(bytes[6] & 0x0F) as usize]);
            output.set(4, HEX_CHARS[(bytes[5] >> 4) as usize]);
            output.set(5, HEX_CHARS[(bytes[5] & 0x0F) as usize]);
            output.set(6, HEX_CHARS[(bytes[4] >> 4) as usize]);
            output.set(7, HEX_CHARS[(bytes[4] & 0x0F) as usize]);
            output.set(8, HEX_CHARS[(bytes[3] >> 4) as usize]);
            output.set(9, HEX_CHARS[(bytes[3] & 0x0F) as usize]);
            output.set(10, HEX_CHARS[(bytes[2] >> 4) as usize]);
            output.set(11, HEX_CHARS[(bytes[2] & 0x0F) as usize]);
            output.set(12, HEX_CHARS[(bytes[1] >> 4) as usize]);
            output.set(13, HEX_CHARS[(bytes[1] & 0x0F) as usize]);
            output.set(14, HEX_CHARS[(bytes[0] >> 4) as usize]);
            output.set(15, HEX_CHARS[(bytes[0] & 0x0F) as usize]);
            output.set_len(16);
        }
    }
}
pub(super) fn validate(text: &str, bytes_count: BytesCount) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0'..=b'9' | b'A'..=b'F' | b'a'..=b'f' ) {
            return ValidateResult::FormatError;
        }
    }
    // check to see if I sould complete
    if buf.len() == (bytes_count as usize) * 2 {
        ValidateResult::Update
    } else {
        ValidateResult::Valid
    }
}
pub(super) fn convert_to_bytes(text: &str, bytes_count: BytesCount) -> ([u8; 8], u8) {
    if let Ok(n) = u64::from_str_radix(text, 16) {
        let bytes = n.to_ne_bytes();
        (bytes, bytes_count as u8)
    } else {
        ([0; 8], 0)
    }
}
