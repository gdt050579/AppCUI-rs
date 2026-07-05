use super::IntFormat;
use super::super::OutputBuffer;
use super::ValidateResult;

fn value_from_bytes(bytes: [u8; 8], format: IntFormat) -> i64 {
    match format {
        IntFormat::I8 => bytes[0] as i8 as i64,
        IntFormat::I16 => i16::from_ne_bytes([bytes[0], bytes[1]]) as i64,
        IntFormat::I32 => i32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64,
        IntFormat::I64 => i64::from_ne_bytes(bytes),
    }
}

fn abs_value(value: i64) -> u64 {
    if value >= 0 {
        value as u64
    } else if value == i64::MIN {
        9223372036854775808u64
    } else {
        (-value) as u64
    }
}

pub(super) fn write(bytes: [u8; 8], format: IntFormat, output: &mut OutputBuffer) {
    let value = value_from_bytes(bytes, format);
    let width = display_chars(format) as usize;
    let sign = if value < 0 { b'-' } else { b'+' };
    let mut v = abs_value(value);
    let mut buf = [b' '; 20];
    let mut pos = width;
    if v == 0 {
        buf[width - 1] = b'0';
        pos = width - 1;
    } else {
        while v > 0 {
            pos -= 1;
            buf[pos] = (v % 10) as u8 + b'0';
            v /= 10;
        }
    }
    pos -= 1;
    buf[pos] = sign;
    for i in 0..width {
        output.set(i, buf[i]);
    }
    output.set_len(width as u8);
}
pub(super) fn validate(text: &str, format: IntFormat) -> ValidateResult {
    let buf = text.as_bytes();
    for b in buf {
        if !matches!(*b, b'0'..=b'9' | b' ' | b'+' | b'-') {
            return ValidateResult::FormatError;
        }
    }
    ValidateResult::Valid
}
pub(super) fn convert_to_bytes(text: &str, format: IntFormat) -> ([u8; 8], u8) {
    if let Ok(n) = text.trim().parse::<i64>() {
        let bytes = n.to_ne_bytes();
        (bytes, format as u8)
    } else {
        ([0; 8], 0)
    }
}
pub(super) fn display_chars(format: IntFormat) -> u32 {
    match format {
        IntFormat::I8 => 4,
        IntFormat::I16 => 6,
        IntFormat::I32 => 11,
        IntFormat::I64 => 20,
    }
}
