use super::super::OutputBuffer;
use super::ValidateResult;


pub(super) fn write(bytes: [u8; 8], output: &mut OutputBuffer) {
    let v = bytes[0];
    for i in 0..8 {
        let bit = (v >> (7 - i)) & 1;
        output.set(i, if bit != 0 { b'1' } else { b'0' });
    }
    output.set_len(8);
}
pub(super) fn validate(text: &str) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0' | b'1') {
            return ValidateResult::FormatError;
        }
    }
    if buf.len() == 8 {
        ValidateResult::Update
    } else {
        ValidateResult::Valid
    }
}
pub(super) fn convert_to_bytes(text: &str) -> ([u8; 8], u8) {
    if let Ok(n) = u64::from_str_radix(text, 2) {
        let bytes = n.to_ne_bytes();
        (bytes, 1)
    } else {
        ([0; 8], 0)
    }
}