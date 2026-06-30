use super::{bin, hex, oct, BytesCount};
use super::super::OutputBuffer;

fn write_hex(bytes: [u8; 8], bytes_count: BytesCount) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    hex::write(bytes, bytes_count, &mut output);
    output.as_slice().to_vec()
}

fn write_oct(bytes: [u8; 8]) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    oct::write(bytes, &mut output);
    output.as_slice().to_vec()
}

fn write_bin(bytes: [u8; 8]) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    bin::write(bytes, &mut output);
    output.as_slice().to_vec()
}

#[test]
fn hex_write_one_byte() {
    assert_eq!(write_hex([0x00, 0, 0, 0, 0, 0, 0, 0], BytesCount::One), b"00");
    assert_eq!(write_hex([0x0A, 0, 0, 0, 0, 0, 0, 0], BytesCount::One), b"0A");
    assert_eq!(write_hex([0xAB, 0, 0, 0, 0, 0, 0, 0], BytesCount::One), b"AB");
    assert_eq!(write_hex([0xFF, 0, 0, 0, 0, 0, 0, 0], BytesCount::One), b"FF");
}

#[test]
fn hex_write_two_bytes() {
    assert_eq!(write_hex([0x12, 0x34, 0, 0, 0, 0, 0, 0], BytesCount::Two), b"1234");
    assert_eq!(write_hex([0x00, 0xFF, 0, 0, 0, 0, 0, 0], BytesCount::Two), b"00FF");
}

#[test]
fn hex_write_four_bytes() {
    assert_eq!(
        write_hex([0x01, 0x23, 0x45, 0x67, 0, 0, 0, 0], BytesCount::Four),
        b"01234567"
    );
}

#[test]
fn hex_write_eight_bytes() {
    assert_eq!(
        write_hex([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF], BytesCount::Eight),
        b"0123456789ABCDEF"
    );
}

#[test]
fn oct_write_three_octal_digits() {
    assert_eq!(write_oct([0x00, 0, 0, 0, 0, 0, 0, 0]), b"000");
    assert_eq!(write_oct([0x01, 0, 0, 0, 0, 0, 0, 0]), b"001");
    assert_eq!(write_oct([0x08, 0, 0, 0, 0, 0, 0, 0]), b"010");
    assert_eq!(write_oct([0x40, 0, 0, 0, 0, 0, 0, 0]), b"100");
    assert_eq!(write_oct([0xAB, 0, 0, 0, 0, 0, 0, 0]), b"253");
    assert_eq!(write_oct([0xFF, 0, 0, 0, 0, 0, 0, 0]), b"377");
}

#[test]
fn bin_write_eight_binary_digits_msb_first() {
    assert_eq!(write_bin([0x00, 0, 0, 0, 0, 0, 0, 0]), b"00000000");
    assert_eq!(write_bin([0x01, 0, 0, 0, 0, 0, 0, 0]), b"00000001");
    assert_eq!(write_bin([0xAB, 0, 0, 0, 0, 0, 0, 0]), b"10101011");
    assert_eq!(write_bin([0xFF, 0, 0, 0, 0, 0, 0, 0]), b"11111111");
}
