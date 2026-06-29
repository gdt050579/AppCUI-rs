use super::search_parser::{parse, Error};

fn parse_ok(text: &str) -> Vec<u8> {
    let mut output = Vec::new();
    parse(text, &mut output).expect("parse should succeed");
    output
}

fn parse_result(text: &str) -> Result<Vec<u8>, Error> {
    let mut output = Vec::new();
    parse(text, &mut output).map(|()| output)
}

#[test]
fn search_parser_empty_input() {
    assert_eq!(parse_result(""), Err(Error::EmptySearch));
    assert_eq!(parse_result("   "), Err(Error::EmptySearch));
}

#[test]
fn search_parser_bare_text() {
    assert_eq!(parse_ok("hello"), b"hello".to_vec());
    assert_eq!(parse_ok("hello world"), b"hello world".to_vec());
    assert_eq!(parse_ok("48 65"), b"48 65".to_vec());
}

#[test]
fn search_parser_bare_quoted_text() {
    assert_eq!(parse_ok(r#""hello""#), b"hello".to_vec());
    assert_eq!(parse_ok(r#"'hello'"#), b"hello".to_vec());
    assert_eq!(parse_ok(r#""   xyz   ""#), b"   xyz   ".to_vec());
    assert_eq!(parse_ok(r#"'   xyz   '"#), b"   xyz   ".to_vec());
    assert_eq!(parse_ok(r#"'say "hi"'"#), br#"say "hi""#.to_vec());
}

#[test]
fn search_parser_text_prefix() {
    assert_eq!(parse_ok("text:hello"), b"hello".to_vec());
    assert_eq!(parse_ok(r#"text:"   xyz   ""#), b"   xyz   ".to_vec());
    assert_eq!(parse_ok("text:'   xyz   '"), b"   xyz   ".to_vec());
    assert_eq!(parse_ok("text:  hello"), b"hello".to_vec());
}

#[test]
fn search_parser_hex_prefix() {
    assert_eq!(parse_ok("hex:68656c6c6f"), b"hello".to_vec());
    assert_eq!(parse_ok("hex:68 65 6c 6c 6f"), b"hello".to_vec());
    assert_eq!(parse_ok("hex:48656c6c6f"), b"Hello".to_vec());
    assert_eq!(parse_ok("hex:FF 00"), vec![0xFF, 0x00]);
    assert_eq!(parse_ok("hex:FF,00"), vec![0xFF, 0x00]);
    assert_eq!(parse_ok("hex:FF   ,   25"), vec![0xFF, 0x25]);
    assert_eq!(parse_ok("hex:ff00"), vec![0xFF, 0x00]);
}

#[test]
fn search_parser_hex_invalid() {
    assert_eq!(parse_result("hex:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("hex:   "), Err(Error::EmptySearch));
    assert_eq!(parse_result("hex:4"), Err(Error::InvalidHex));
    assert_eq!(parse_result("hex:GG"), Err(Error::InvalidHex));
    assert_eq!(parse_result("hex:48 6Z"), Err(Error::InvalidHex));
}

#[test]
fn search_parser_text_invalid_quote() {
    assert_eq!(parse_result(r#"""#), Err(Error::UnclosedQuote));
    assert_eq!(parse_result(r#"text:"hello"#), Err(Error::UnclosedQuote));
    assert_eq!(parse_result(r#"'hello"#), Err(Error::UnclosedQuote));
    assert_eq!(parse_result(r#""hello" trailing"#), Err(Error::InvalidText));
}

#[test]
fn search_parser_u8_prefix() {
    assert_eq!(parse_ok("u8:0,255,128"), vec![0, 255, 128]);
    assert_eq!(parse_ok("u8:10, 20,30"), vec![10, 20, 30]);
    assert_eq!(parse_ok("u8:  1   ,   2  "), vec![1, 2]);
    assert_eq!(parse_ok("u8:255"), vec![255]);
}

#[test]
fn search_parser_u8_invalid() {
    assert_eq!(parse_result("u8:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("u8:  ,  "), Err(Error::EmptySearch));
    assert_eq!(parse_result("u8:256"), Err(Error::InvalidU8));
    assert_eq!(parse_result("u8:255,256"), Err(Error::InvalidU8));
    assert_eq!(parse_result("u8:12a"), Err(Error::InvalidU8));
    assert_eq!(parse_result("u8:1000"), Err(Error::InvalidU8));
}

#[test]
fn search_parser_i8_prefix() {
    assert_eq!(parse_ok("i8:0,-1,127,-128"), vec![0, 255, 127, 128]);
    assert_eq!(parse_ok("i8:-128, 127"), vec![128, 127]);
    assert_eq!(parse_ok("i8:  10 , -20  "), vec![10, 236]);
}

#[test]
fn search_parser_i8_invalid() {
    assert_eq!(parse_result("i8:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("i8:  ,  "), Err(Error::EmptySearch));
    assert_eq!(parse_result("i8:128"), Err(Error::InvalidI8));
    assert_eq!(parse_result("i8:-129"), Err(Error::InvalidI8));
    assert_eq!(parse_result("i8:-"), Err(Error::InvalidI8));
    assert_eq!(parse_result("i8:12a"), Err(Error::InvalidI8));
}

#[test]
fn search_parser_u16_prefix() {
    assert_eq!(parse_ok("u16:0,300"), vec![0, 0, 0x2C, 0x01]);
    assert_eq!(parse_ok("u16:65535"), 65535u16.to_le_bytes().to_vec());
}

#[test]
fn search_parser_u16_invalid() {
    assert_eq!(parse_result("u16:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("u16:65536"), Err(Error::InvalidU16));
}

#[test]
fn search_parser_i16_prefix() {
    assert_eq!(parse_ok("i16:0,-1"), vec![0, 0, 0xFF, 0xFF]);
    assert_eq!(parse_ok("i16:-32768, 32767"), [-32768i16, 32767i16].iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>());
}

#[test]
fn search_parser_i16_invalid() {
    assert_eq!(parse_result("i16:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("i16:32768"), Err(Error::InvalidI16));
    assert_eq!(parse_result("i16:-32769"), Err(Error::InvalidI16));
}

#[test]
fn search_parser_u32_prefix() {
    assert_eq!(parse_ok("u32:0,305419896"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0u32.to_le_bytes());
        bytes.extend_from_slice(&305419896u32.to_le_bytes());
        bytes
    });
}

#[test]
fn search_parser_u32_invalid() {
    assert_eq!(parse_result("u32:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("u32:4294967296"), Err(Error::InvalidU32));
}

#[test]
fn search_parser_i32_prefix() {
    assert_eq!(parse_ok("i32:1,-1"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1i32.to_le_bytes());
        bytes.extend_from_slice(&(-1i32).to_le_bytes());
        bytes
    });
}

#[test]
fn search_parser_i32_invalid() {
    assert_eq!(parse_result("i32:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("i32:2147483648"), Err(Error::InvalidI32));
    assert_eq!(parse_result("i32:-2147483649"), Err(Error::InvalidI32));
}

#[test]
fn search_parser_u64_prefix() {
    assert_eq!(parse_ok("u64:0,1"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0u64.to_le_bytes());
        bytes.extend_from_slice(&1u64.to_le_bytes());
        bytes
    });
    assert_eq!(parse_ok("u64:18446744073709551615"), u64::MAX.to_le_bytes().to_vec());
}

#[test]
fn search_parser_u64_invalid() {
    assert_eq!(parse_result("u64:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("u64:18446744073709551616"), Err(Error::InvalidU64));
}

#[test]
fn search_parser_i64_prefix() {
    assert_eq!(parse_ok("i64:1,-1"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1i64.to_le_bytes());
        bytes.extend_from_slice(&(-1i64).to_le_bytes());
        bytes
    });
}

#[test]
fn search_parser_i64_invalid() {
    assert_eq!(parse_result("i64:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("i64:9223372036854775808"), Err(Error::InvalidI64));
    assert_eq!(parse_result("i64:-9223372036854775809"), Err(Error::InvalidI64));
}

#[test]
fn search_parser_f32_prefix() {
    assert_eq!(parse_ok("f32:0,1.5,-2.25"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0f32.to_le_bytes());
        bytes.extend_from_slice(&1.5f32.to_le_bytes());
        bytes.extend_from_slice(&(-2.25f32).to_le_bytes());
        bytes
    });
}

#[test]
fn search_parser_f32_invalid() {
    assert_eq!(parse_result("f32:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("f32:not_a_number"), Err(Error::InvalidF32));
}

#[test]
fn search_parser_f64_prefix() {
    assert_eq!(parse_ok("f64:3.14"), 3.14f64.to_le_bytes().to_vec());
    assert_eq!(parse_ok("f64:0, -1.0"), {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0f64.to_le_bytes());
        bytes.extend_from_slice(&(-1.0f64).to_le_bytes());
        bytes
    });
}

#[test]
fn search_parser_f64_invalid() {
    assert_eq!(parse_result("f64:"), Err(Error::EmptySearch));
    assert_eq!(parse_result("f64:1.2.3"), Err(Error::InvalidF64));
}

#[test]
fn search_parser_utf16_prefix() {
    assert_eq!(parse_ok("utf16:ABC"), vec![0x41, 0x00, 0x42, 0x00, 0x43, 0x00]);
    assert_eq!(parse_ok(r#"utf16:"ABC""#), vec![0x41, 0x00, 0x42, 0x00, 0x43, 0x00]);
    assert_eq!(parse_ok("utf16:  hello"), {
        let mut bytes = Vec::new();
        for ch in "hello".chars() {
            bytes.extend_from_slice(&(ch as u16).to_le_bytes());
        }
        bytes
    });
}

#[test]
fn search_parser_utf16_invalid() {
    assert_eq!(parse_result("utf16:"), Err(Error::EmptySearch));
    assert_eq!(parse_result(r##"utf16:"""##), Err(Error::EmptySearch));
    assert_eq!(parse_result(r#"utf16:"abc" trailing"#), Err(Error::InvalidText));
}

#[test]
fn search_parser_colon_in_text_is_not_hex() {
    assert_eq!(parse_ok("http://example.com"), b"http://example.com".to_vec());
}
