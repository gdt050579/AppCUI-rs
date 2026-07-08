use super::super::OutputBuffer;
use super::ValidateResult;

pub(super) fn write(bytes: [u8;8], output: &mut OutputBuffer) {
    let v = bytes[0] as u32;
    output.set(0, ((v >> 6) & 0x07) as u8 + b'0');
    output.set(1, ((v >> 3) & 0x07) as u8 + b'0');
    output.set(2, (v & 0x07) as u8 + b'0');
    output.set_len(3);
}
pub(super) fn validate(text: &str) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0'..=b'7') {
            return ValidateResult::FormatError;
        }
    }
    if buf.len() == 3 {
        ValidateResult::Update
    } else {
        ValidateResult::Valid
    }
}
pub(super) fn convert_to_bytes(text: &str) -> ([u8; 8], u8) {
    if let Ok(n) = u64::from_str_radix(text, 8) {
        let bytes = n.to_ne_bytes();
        (bytes, 1)
    } else {
        ([0; 8], 0)
    }
}