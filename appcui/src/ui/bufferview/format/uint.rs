use super::super::BytesCount;
use super::super::OutputBuffer;
use super::ValidateResult;

fn value_from_bytes(bytes: [u8; 8], bytes_count: BytesCount) -> u64 {
    match bytes_count {
        BytesCount::One => bytes[0] as u64,
        BytesCount::Two => u16::from_ne_bytes([bytes[0], bytes[1]]) as u64,
        BytesCount::Four => u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64,
        BytesCount::Eight => u64::from_ne_bytes(bytes),
    }
}

pub(super) fn write(bytes: [u8; 8], bytes_count: BytesCount, output: &mut OutputBuffer) {
    let value = value_from_bytes(bytes, bytes_count);
    let width = display_chars(bytes_count) as usize;
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
pub(super) fn validate(text: &str, bytes_count: BytesCount) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0'..=b'9' | b' ') {
            return ValidateResult::FormatError;
        }
    }
    ValidateResult::Valid
}
pub(super) fn convert_to_bytes(text: &str, bytes_count: BytesCount) -> ([u8; 8], u8) {
    if let Ok(n) = text.trim().parse::<u64>() {
        let bytes = n.to_ne_bytes();
        (bytes, bytes_count as u8)
    } else {
        ([0; 8], 0)
    }
}
pub(super) fn display_chars(bytes_count: BytesCount) -> u32 {
    match bytes_count {
        BytesCount::One => 3,
        BytesCount::Two => 5,
        BytesCount::Four => 10,
        BytesCount::Eight => 20,
    }
}
