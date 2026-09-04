use super::super::OutputBuffer;
use super::{
    bin, float, hex, int, oct, uint, DataRepresentationFormat, FloatFormat, HexFormat, IntFormat,
    UIntFormat, ValidateResult,
};

fn f32_bytes(v: f32) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(&v.to_le_bytes());
    bytes
}

fn f64_bytes(v: f64) -> [u8; 8] {
    v.to_le_bytes()
}

fn extend_bytes<const N: usize>(prefix: [u8; N]) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..N].copy_from_slice(&prefix);
    bytes
}

fn write_format(bytes: [u8; 8], format: DataRepresentationFormat) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    format.write(bytes, &mut output);
    output.as_slice().to_vec()
}

fn write_hex(bytes: [u8; 8], format: HexFormat) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    hex::write(bytes, format, &mut output);
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

fn write_int(bytes: [u8; 8], format: IntFormat) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    int::write(bytes, format, &mut output);
    output.as_slice().to_vec()
}

fn write_uint(bytes: [u8; 8], format: UIntFormat) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    uint::write(bytes, format, &mut output);
    output.as_slice().to_vec()
}

fn write_float(bytes: [u8; 8], format: FloatFormat) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    float::write(bytes, format, &mut output);
    output.as_slice().to_vec()
}

#[test]
fn hex_write_one_byte() {
    assert_eq!(write_hex([0x00, 0, 0, 0, 0, 0, 0, 0], HexFormat::Byte), b"00");
    assert_eq!(write_hex([0x0A, 0, 0, 0, 0, 0, 0, 0], HexFormat::Byte), b"0A");
    assert_eq!(write_hex([0xAB, 0, 0, 0, 0, 0, 0, 0], HexFormat::Byte), b"AB");
    assert_eq!(write_hex([0xFF, 0, 0, 0, 0, 0, 0, 0], HexFormat::Byte), b"FF");
}

#[test]
fn hex_write_two_bytes() {
    assert_eq!(write_hex([0x34, 0x12, 0, 0, 0, 0, 0, 0], HexFormat::Word), b"1234");
    assert_eq!(write_hex([0xFF, 0x00, 0, 0, 0, 0, 0, 0], HexFormat::Word), b"00FF");
}

#[test]
fn hex_write_four_bytes() {
    assert_eq!(write_hex([0x67, 0x45, 0x23, 0x01, 0, 0, 0, 0], HexFormat::DWord), b"01234567");
}

#[test]
fn hex_write_eight_bytes() {
    assert_eq!(
        write_hex([0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01], HexFormat::QWord),
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
    assert_eq!(hex::validate("A", HexFormat::Byte), ValidateResult::Valid);
    assert_eq!(hex::validate("AB", HexFormat::Byte), ValidateResult::Update);
    assert_eq!(hex::validate("ab", HexFormat::Byte), ValidateResult::Update);
    assert_eq!(hex::validate("12", HexFormat::Word), ValidateResult::Valid);
    assert_eq!(hex::validate("1234", HexFormat::Word), ValidateResult::Update);
}

#[test]
fn hex_validate_rejects_invalid_digits() {
    assert_eq!(hex::validate("GH", HexFormat::Byte), ValidateResult::FormatError);
    assert_eq!(hex::validate("12G4", HexFormat::Word), ValidateResult::FormatError);
}

#[test]
fn hex_convert_to_bytes_parses_by_bytes_count() {
    let (bytes, count) = hex::convert_to_bytes("AB", HexFormat::Byte);
    assert_eq!(count, HexFormat::Byte as u8);
    assert_eq!(bytes, 0xAB_u64.to_ne_bytes());

    let (bytes, count) = hex::convert_to_bytes("1234", HexFormat::Word);
    assert_eq!(count, HexFormat::Word as u8);
    assert_eq!(bytes, 0x1234_u64.to_ne_bytes());
}

#[test]
fn hex_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(hex::convert_to_bytes("GG", HexFormat::Byte), ([0; 8], 0));
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

#[test]
fn int_write_signed_values() {
    assert_eq!(write_int([0x00, 0, 0, 0, 0, 0, 0, 0], IntFormat::I8), b"  +0");
    assert_eq!(write_int([0x7F, 0, 0, 0, 0, 0, 0, 0], IntFormat::I8), b"+127");
    assert_eq!(write_int([0x80, 0, 0, 0, 0, 0, 0, 0], IntFormat::I8), b"-128");
    assert_eq!(write_int([0x00, 0x01, 0, 0, 0, 0, 0, 0], IntFormat::I16), b"  +256");
    assert_eq!(write_int([0x00, 0x80, 0, 0, 0, 0, 0, 0], IntFormat::I16), b"-32768");
}

#[test]
fn int_validate_accepts_signed_decimal_digits() {
    assert_eq!(int::validate("123", IntFormat::I32), ValidateResult::Valid);
    assert_eq!(int::validate("-42", IntFormat::I32), ValidateResult::Valid);
    assert_eq!(int::validate("+7", IntFormat::I8), ValidateResult::Valid);
    assert_eq!(int::validate("  99 ", IntFormat::I16), ValidateResult::Valid);
}

#[test]
fn int_validate_rejects_invalid_digits() {
    assert_eq!(int::validate("12a", IntFormat::I32), ValidateResult::FormatError);
    assert_eq!(int::validate("1.5", IntFormat::I32), ValidateResult::FormatError);
}

#[test]
fn int_convert_to_bytes_parses_by_format() {
    let (bytes, count) = int::convert_to_bytes("-128", IntFormat::I8);
    assert_eq!(count, IntFormat::I8 as u8);
    assert_eq!(bytes, (-128_i64).to_ne_bytes());

    let (bytes, count) = int::convert_to_bytes("256", IntFormat::I16);
    assert_eq!(count, IntFormat::I16 as u8);
    assert_eq!(bytes, 256_i64.to_ne_bytes());
}

#[test]
fn int_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(int::convert_to_bytes("not-a-number", IntFormat::I32), ([0; 8], 0));
}

#[test]
fn uint_write_unsigned_values() {
    assert_eq!(write_uint([0x00, 0, 0, 0, 0, 0, 0, 0], UIntFormat::U8), b"  0");
    assert_eq!(write_uint([0xFF, 0, 0, 0, 0, 0, 0, 0], UIntFormat::U8), b"255");
    assert_eq!(write_uint([0x00, 0x01, 0, 0, 0, 0, 0, 0], UIntFormat::U16), b"  256");
    assert_eq!(write_uint([0xFF, 0xFF, 0, 0, 0, 0, 0, 0], UIntFormat::U16), b"65535");
}

#[test]
fn uint_validate_accepts_decimal_digits() {
    assert_eq!(uint::validate("255", UIntFormat::U8), ValidateResult::Valid);
    assert_eq!(uint::validate(" 42 ", UIntFormat::U16), ValidateResult::Valid);
}

#[test]
fn uint_validate_rejects_invalid_digits() {
    assert_eq!(uint::validate("-1", UIntFormat::U8), ValidateResult::FormatError);
    assert_eq!(uint::validate("12a", UIntFormat::U32), ValidateResult::FormatError);
}

#[test]
fn uint_convert_to_bytes_parses_by_format() {
    let (bytes, count) = uint::convert_to_bytes("255", UIntFormat::U8);
    assert_eq!(count, UIntFormat::U8 as u8);
    assert_eq!(bytes, 255_u64.to_ne_bytes());

    let (bytes, count) = uint::convert_to_bytes("65535", UIntFormat::U16);
    assert_eq!(count, UIntFormat::U16 as u8);
    assert_eq!(bytes, 65535_u64.to_ne_bytes());
}

#[test]
fn uint_convert_to_bytes_returns_zero_on_parse_failure() {
    assert_eq!(uint::convert_to_bytes("not-a-number", UIntFormat::U32), ([0; 8], 0));
}

#[test]
fn e4m3_write_fixed_decimal() {
    assert_eq!(write_float([0x00, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E4M3), b"+000.000");
    assert_eq!(write_float([0x38, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E4M3), b"+001.000");
    assert_eq!(write_float([0x77, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E4M3), b"+240.000");
    assert_eq!(write_float([0xF7, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E4M3), b"-240.000");
    assert_eq!(write_float([0x7F, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E4M3), b"     NaN");
}

#[test]
fn e5m2_write_fixed_decimal() {
    assert_eq!(write_float([0x00, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E5M2), b"+00000.000000");
    assert_eq!(write_float([0x7C, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E5M2), b"          inf");
    assert_eq!(write_float([0xFC, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E5M2), b"         -inf");
    assert_eq!(write_float([0x7F, 0, 0, 0, 0, 0, 0, 0], FloatFormat::E5M2), b"          NaN");
}

#[test]
fn fp8_validate_rejects_scientific_notation() {
    assert_eq!(float::validate("1.2e3", FloatFormat::E4M3), ValidateResult::FormatError);
    assert_eq!(float::validate("1.2e3", FloatFormat::E5M2), ValidateResult::FormatError);
    assert_eq!(float::validate("+240.000", FloatFormat::E4M3), ValidateResult::Valid);
}

#[test]
fn e4m3_convert_to_bytes_roundtrip() {
    let (bytes, count) = float::convert_to_bytes("+240.000", FloatFormat::E4M3);
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 0x77);
    assert_eq!(write_float(bytes, FloatFormat::E4M3), b"+240.000");
}

#[test]
fn hex_convert_to_bytes_parses_dword_and_qword() {
    let (bytes, count) = hex::convert_to_bytes("01234567", HexFormat::DWord);
    assert_eq!(count, HexFormat::DWord as u8);
    assert_eq!(bytes, 0x0123_4567_u64.to_ne_bytes());

    let (bytes, count) = hex::convert_to_bytes("0123456789ABCDEF", HexFormat::QWord);
    assert_eq!(count, HexFormat::QWord as u8);
    assert_eq!(bytes, 0x0123_4567_89AB_CDEF_u64.to_ne_bytes());
}

#[test]
fn int_write_i32_and_i64() {
    assert_eq!(
        write_int(extend_bytes(1000_i32.to_ne_bytes()), IntFormat::I32),
        b"      +1000"
    );
    assert_eq!(
        write_int(extend_bytes(i32::MAX.to_ne_bytes()), IntFormat::I32),
        b"+2147483647"
    );
    assert_eq!(
        write_int(extend_bytes(i32::MIN.to_ne_bytes()), IntFormat::I32),
        b"-2147483648"
    );
    assert_eq!(
        write_int(1_000_000_i64.to_ne_bytes(), IntFormat::I64),
        b"            +1000000"
    );
}

#[test]
fn uint_write_u32_and_u64() {
    assert_eq!(
        write_uint(extend_bytes(1_000_000_u32.to_ne_bytes()), UIntFormat::U32),
        b"   1000000"
    );
    assert_eq!(
        write_uint(extend_bytes(u32::MAX.to_ne_bytes()), UIntFormat::U32),
        b"4294967295"
    );
    assert_eq!(
        write_uint(u64::MAX.to_ne_bytes(), UIntFormat::U64),
        b"18446744073709551615"
    );
}

#[test]
fn scientific32_write_values() {
    assert_eq!(write_float(f32_bytes(0.0), FloatFormat::Scientific32), b"+0.00e+00");
    assert_eq!(write_float(f32_bytes(1.0), FloatFormat::Scientific32), b"+1.00e+00");
    assert_eq!(write_float(f32_bytes(-2.5), FloatFormat::Scientific32), b"-2.50e+00");
    assert_eq!(
        write_float(f32_bytes(f32::INFINITY), FloatFormat::Scientific32),
        b"      inf"
    );
    assert_eq!(
        write_float(f32_bytes(f32::NEG_INFINITY), FloatFormat::Scientific32),
        b"     -inf"
    );
    assert_eq!(write_float(f32_bytes(f32::NAN), FloatFormat::Scientific32), b"      NaN");
}

#[test]
fn scientific64_write_values() {
    assert_eq!(write_float(f64_bytes(0.0), FloatFormat::Scientific64), b"+0.000000000000000e+000");
    assert_eq!(
        write_float(f64_bytes(1.0), FloatFormat::Scientific64),
        b"+1.000000000000000e+000"
    );
    assert_eq!(
        write_float(f64_bytes(-2.5), FloatFormat::Scientific64),
        b"-2.500000000000000e+000"
    );
    assert_eq!(
        write_float(f64_bytes(f64::INFINITY), FloatFormat::Scientific64),
        b"                    inf"
    );
    assert_eq!(
        write_float(f64_bytes(f64::NEG_INFINITY), FloatFormat::Scientific64),
        b"                   -inf"
    );
    assert_eq!(
        write_float(f64_bytes(f64::NAN), FloatFormat::Scientific64),
        b"                    NaN"
    );
}

#[test]
fn scientific_validate_accepts_float_prefixes() {
    assert_eq!(
        float::validate("1.2", FloatFormat::Scientific32),
        ValidateResult::Valid
    );
    assert_eq!(
        float::validate("1.2e3", FloatFormat::Scientific32),
        ValidateResult::Valid
    );
    assert_eq!(
        float::validate("-inf", FloatFormat::Scientific64),
        ValidateResult::Valid
    );
    assert_eq!(float::validate("nan", FloatFormat::Scientific64), ValidateResult::Valid);
}

#[test]
fn scientific_validate_rejects_invalid_prefixes() {
    assert_eq!(
        float::validate("1.2.3", FloatFormat::Scientific32),
        ValidateResult::FormatError
    );
    assert_eq!(
        float::validate("abc", FloatFormat::Scientific64),
        ValidateResult::FormatError
    );
}

#[test]
fn scientific32_convert_to_bytes_roundtrip() {
    let (bytes, count) = float::convert_to_bytes("1.5", FloatFormat::Scientific32);
    assert_eq!(count, 4);
    assert_eq!(bytes[..4], 1.5_f32.to_le_bytes());
    assert_eq!(write_float(bytes, FloatFormat::Scientific32), b"+1.50e+00");
}

#[test]
fn scientific64_convert_to_bytes_roundtrip() {
    let (bytes, count) = float::convert_to_bytes("-2.5", FloatFormat::Scientific64);
    assert_eq!(count, 8);
    assert_eq!(bytes, (-2.5_f64).to_le_bytes());
    assert_eq!(write_float(bytes, FloatFormat::Scientific64), b"-2.500000000000000e+000");
}

#[test]
fn e5m2_convert_to_bytes_roundtrip() {
    let (bytes, count) = float::convert_to_bytes("+57344.000000", FloatFormat::E5M2);
    assert_eq!(count, 1);
    assert_eq!(write_float(bytes, FloatFormat::E5M2), b"+57344.000000");
}

#[test]
fn fp8_validate_accepts_inf_and_nan_prefixes() {
    assert_eq!(float::validate("inf", FloatFormat::E5M2), ValidateResult::Valid);
    assert_eq!(float::validate("NaN", FloatFormat::E4M3), ValidateResult::Valid);
}

#[test]
fn data_representation_format_char() {
    let format = DataRepresentationFormat::Char;
    assert!(format.is_char());
    assert_eq!(write_format([b'A', 0, 0, 0, 0, 0, 0, 0], format), b"A");
    assert_eq!(format.validate(""), ValidateResult::Valid);
    assert_eq!(format.validate("Z"), ValidateResult::Update);
    let (bytes, count) = format.convert_to_bytes("Hi");
    assert_eq!(count, 2);
    assert_eq!(&bytes[..2], b"Hi");
}

#[test]
fn data_representation_format_empty_input() {
    let format = DataRepresentationFormat::Hex(HexFormat::Byte);
    assert_eq!(format.validate(""), ValidateResult::Valid);
    assert_eq!(format.convert_to_bytes(""), ([0; 8], 0));
}

#[test]
fn data_representation_format_bytes_count_and_display_chars() {
    assert_eq!(
        DataRepresentationFormat::Hex(HexFormat::Word).bytes_count(),
        2
    );
    assert_eq!(
        DataRepresentationFormat::Hex(HexFormat::Word).display_chars(),
        4
    );
    assert_eq!(
        DataRepresentationFormat::Float(FloatFormat::Scientific32).bytes_count(),
        4
    );
    assert_eq!(
        DataRepresentationFormat::Float(FloatFormat::Scientific64).display_chars(),
        23
    );
    assert_eq!(DataRepresentationFormat::Oct.bytes_count(), 1);
    assert_eq!(DataRepresentationFormat::Bin.display_chars(), 8);
    assert_eq!(DataRepresentationFormat::Char.display_chars(), 1);
}
    