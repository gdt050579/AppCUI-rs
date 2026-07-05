use super::{bin, hex, oct, BytesCount, ValidateResult};
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
    assert_eq!(write_hex([0x34, 0x12, 0, 0, 0, 0, 0, 0], BytesCount::Two), b"1234");
    assert_eq!(write_hex([0xFF, 0x00, 0, 0, 0, 0, 0, 0], BytesCount::Two), b"00FF");
}

#[test]
fn hex_write_four_bytes() {
    assert_eq!(
        write_hex([0x67, 0x45, 0x23, 0x01, 0, 0, 0, 0], BytesCount::Four),
        b"01234567"
    );
}

#[test]
fn hex_write_eight_bytes() {
    assert_eq!(
        write_hex([0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01], BytesCount::Eight),
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

#[test]
fn bin_validate_accepts_binary_digits() {
    assert_eq!(bin::validate("101"), ValidateResult::Valid);
    assert_eq!(bin::validate("10101011"), ValidateResult::Update);
    assert_eq!(bin::validate("00000000"), ValidateResult::Update);
}

#[test]
fn bin_validate_rejects_invalid_digits() {
    assert_eq!(bin::validate("1012"), ValidateResult::FormatError);
    assert_eq!(bin::validate("abcdefgh"), ValidateResult::FormatError);
}

#[test]
fn bin_convert_to_bytes_parses_eight_bit_value() {
    let (bytes, count) = bin::convert_to_bytes("10101011");
    assert_eq!(count, 1);
    assert_eq!(bytes, 0xAB_u64.to_ne_bytes());

    let (bytes, count) = bin::convert_to_bytes("11111111");
    assert_eq!(count, 1);
    assert_eq!(bytes, 0xFF_u64.to_ne_bytes());
}

#[test]
fn bin_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(bin::convert_to_bytes("not-binary"), ([0; 8], 0));
}

#[test]
fn hex_validate_accepts_hex_digits() {
    assert_eq!(hex::validate("A", BytesCount::One), ValidateResult::Valid);
    assert_eq!(hex::validate("AB", BytesCount::One), ValidateResult::Update);
    assert_eq!(hex::validate("ab", BytesCount::One), ValidateResult::Update);
    assert_eq!(hex::validate("12", BytesCount::Two), ValidateResult::Valid);
    assert_eq!(hex::validate("1234", BytesCount::Two), ValidateResult::Update);
}

#[test]
fn hex_validate_rejects_invalid_digits() {
    assert_eq!(hex::validate("GH", BytesCount::One), ValidateResult::FormatError);
    assert_eq!(hex::validate("12G4", BytesCount::Two), ValidateResult::FormatError);
}

#[test]
fn hex_convert_to_bytes_parses_by_bytes_count() {
    let (bytes, count) = hex::convert_to_bytes("AB", BytesCount::One);
    assert_eq!(count, BytesCount::One as u8);
    assert_eq!(bytes, 0xAB_u64.to_ne_bytes());

    let (bytes, count) = hex::convert_to_bytes("1234", BytesCount::Two);
    assert_eq!(count, BytesCount::Two as u8);
    assert_eq!(bytes, 0x1234_u64.to_ne_bytes());
}

#[test]
fn hex_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(hex::convert_to_bytes("GG", BytesCount::One), ([0; 8], 0));
}

#[test]
fn oct_validate_accepts_octal_digits() {
    assert_eq!(oct::validate("25"), ValidateResult::Valid);
    assert_eq!(oct::validate("253"), ValidateResult::Update);
    assert_eq!(oct::validate("377"), ValidateResult::Update);
}

#[test]
fn oct_validate_rejects_invalid_digits() {
    assert_eq!(oct::validate("28"), ValidateResult::FormatError);
    assert_eq!(oct::validate("abc"), ValidateResult::FormatError);
}

#[test]
fn oct_convert_to_bytes_parses_three_digit_value() {
    let (bytes, count) = oct::convert_to_bytes("253");
    assert_eq!(count, 1);
    assert_eq!(bytes, 0xAB_u64.to_ne_bytes());

    let (bytes, count) = oct::convert_to_bytes("377");
    assert_eq!(count, 1);
    assert_eq!(bytes, 0xFF_u64.to_ne_bytes());
}

#[test]
fn oct_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(oct::convert_to_bytes("999"), ([0; 8], 0));
}
