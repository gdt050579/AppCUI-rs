use super::UIntFormat;
use super::super::OutputBuffer;
use super::ValidateResult;

fn value_from_bytes(bytes: [u8; 8], format: UIntFormat) -> u64 {
    match format {
        UIntFormat::U8 => bytes[0] as u64,
        UIntFormat::U16 => u16::from_ne_bytes([bytes[0], bytes[1]]) as u64,
        UIntFormat::U32 => u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64,
        UIntFormat::U64 => u64::from_ne_bytes(bytes),
    }
}

pub(super) fn write(bytes: [u8; 8], format: UIntFormat, output: &mut OutputBuffer) {
    let value = value_from_bytes(bytes, format);
    let width = display_chars(format) as usize;
    let mut buf = [b' '; 20];
    let mut v = value;
    let mut pos = width;
    if v == 0 {
        buf[width - 1] = b'0';
    } else {
        while v > 0 {
            pos -= 1;
            buf[pos] = (v % 10) as u8 + b'0';
            v /= 10;
        }
    }
    for i in 0..width {
        output.set(i, buf[i]);
    }
    output.set_len(width as u8);
}
pub(super) fn validate(text: &str, format: UIntFormat) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0'..=b'9' | b' ') {
            return ValidateResult::FormatError;
        }
    }
    ValidateResult::Valid
}
pub(super) fn convert_to_bytes(text: &str, format: UIntFormat) -> ([u8; 8], u8) {
    if let Ok(n) = text.trim().parse::<u64>() {
        let bytes = n.to_ne_bytes();
        (bytes, format as u8)
    } else {
        ([0; 8], 0)
    }
}
pub(super) fn display_chars(format: UIntFormat) -> u32 {
    match format {
        UIntFormat::U8 => 3,
        UIntFormat::U16 => 5,
        UIntFormat::U32 => 10,
        UIntFormat::U64 => 20,
    }
}
