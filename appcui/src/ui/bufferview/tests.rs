use super::buffer::{Buffer, BufferAccess};
use super::bufferview::BufferView;
use super::search_parser::{parse, Error};

fn buffer(data: impl Into<Vec<u8>>) -> Buffer<Vec<u8>> {
    Buffer::new(data.into())
}

fn snapshot(buf: &mut Buffer<Vec<u8>>) -> Vec<u8> {
    let len = buf.len() as usize;
    let mut out = vec![0u8; len];
    buf.read_bytes(0, &mut out);
    out
}

struct ReadOnlyBuffer {
    data: Vec<u8>,
}

impl BufferAccess for ReadOnlyBuffer {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if pos < self.data.len() as u64 {
            Some(self.data[pos as usize])
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        false
    }

    fn set(&mut self, _pos: u64, _value: u8) -> bool {
        false
    }

    fn can_resize(&self) -> bool {
        false
    }

    fn resize(&mut self, _new_size: u64) -> bool {
        false
    }
}

struct ResizeFailsBuffer {
    data: Vec<u8>,
}

impl BufferAccess for ResizeFailsBuffer {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if pos < self.data.len() as u64 {
            Some(self.data[pos as usize])
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        true
    }

    fn set(&mut self, pos: u64, value: u8) -> bool {
        if pos < self.data.len() as u64 {
            self.data[pos as usize] = value;
            true
        } else {
            false
        }
    }

    fn can_resize(&self) -> bool {
        true
    }

    fn resize(&mut self, _new_size: u64) -> bool {
        false
    }
}

struct SetFailsAtBuffer {
    data: Vec<u8>,
    fail_at: u64,
}

impl BufferAccess for SetFailsAtBuffer {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if pos < self.data.len() as u64 {
            Some(self.data[pos as usize])
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        true
    }

    fn set(&mut self, pos: u64, value: u8) -> bool {
        if pos == self.fail_at {
            return false;
        }
        if pos < self.data.len() as u64 {
            self.data[pos as usize] = value;
            true
        } else {
            false
        }
    }

    fn can_resize(&self) -> bool {
        true
    }

    fn resize(&mut self, new_size: u64) -> bool {
        self.data.resize(new_size as usize, 0);
        true
    }
}

struct GetFailsAtBuffer {
    data: Vec<u8>,
    fail_at: u64,
}

impl BufferAccess for GetFailsAtBuffer {
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    fn get(&mut self, pos: u64) -> Option<u8> {
        if pos == self.fail_at {
            return None;
        }
        if pos < self.data.len() as u64 {
            Some(self.data[pos as usize])
        } else {
            None
        }
    }

    fn can_write(&self) -> bool {
        true
    }

    fn set(&mut self, pos: u64, value: u8) -> bool {
        if pos < self.data.len() as u64 {
            self.data[pos as usize] = value;
            true
        } else {
            false
        }
    }

    fn can_resize(&self) -> bool {
        true
    }

    fn resize(&mut self, new_size: u64) -> bool {
        self.data.resize(new_size as usize, 0);
        true
    }
}

#[test]
fn buffer_read_bytes_exact_zero_length() {
    let mut buf = buffer(b"abc".to_vec());
    let mut out = [0u8; 0];
    assert!(buf.read_bytes_exact(0, &mut out));
    assert!(buf.read_bytes_exact(3, &mut out));
    assert!(!buf.read_bytes_exact(4, &mut out));
}

#[test]
fn buffer_delete_fails_when_final_resize_fails() {
    let mut buf = Buffer::new(ResizeFailsBuffer {
        data: b"hello".to_vec(),
    });
    assert!(!buf.delete(0, 1));
}

#[test]
fn buffer_delete_fails_when_shift_overwrite_fails() {
    let mut buf = Buffer::new(SetFailsAtBuffer {
        data: b"0123456789".to_vec(),
        fail_at: 1,
    });
    assert!(!buf.delete(0, 2));
}

#[test]
fn buffer_insert_fails_when_shift_read_fails() {
    let mut buf = Buffer::new(GetFailsAtBuffer {
        data: b"0123456789".to_vec(),
        fail_at: 2,
    });
    assert!(!buf.insert(1, b"XY"));
}

#[test]
fn buffer_insert_fails_when_shift_overwrite_fails() {
    let mut buf = Buffer::new(SetFailsAtBuffer {
        data: b"0123456789".to_vec(),
        fail_at: 5,
    });
    assert!(!buf.insert(1, b"XY"));
}

#[test]
fn buffer_write_bytes_empty_slice() {
    let mut buf = buffer(b"abc".to_vec());
    assert!(buf.write_bytes(0, b""));
    assert!(buf.write_bytes(3, b""));
    assert_eq!(snapshot(&mut buf), b"abc".to_vec());
}

#[test]
fn buffer_vec_access_len_and_get() {
    let mut empty = buffer(vec![]);
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.get(0), None);

    let mut data = buffer(b"hello".to_vec());
    assert_eq!(data.len(), 5);
    assert_eq!(data.get(0), Some(b'h'));
    assert_eq!(data.get(4), Some(b'o'));
    assert_eq!(data.get(5), None);
    assert_eq!(data.get(u64::MAX), None);
}

#[test]
fn buffer_vec_access_can_edit_and_resize() {
    let editable = buffer(vec![1, 2, 3]);
    assert!(editable.can_edit());
    assert!(editable.can_resize());
}

#[test]
fn buffer_read_bytes_exact_success_and_bounds() {
    let mut buf = buffer(b"ABCDEF".to_vec());

    let mut out = [0u8; 3];
    assert!(buf.read_bytes_exact(0, &mut out));
    assert_eq!(&out, b"ABC");

    let mut out = [0u8; 2];
    assert!(buf.read_bytes_exact(4, &mut out));
    assert_eq!(&out, b"EF");

    let mut out = [0u8; 1];
    assert!(buf.read_bytes_exact(5, &mut out));
    assert_eq!(&out, b"F");

    let mut out = [0u8; 4];
    assert!(!buf.read_bytes_exact(3, &mut out));

    let mut out = [0u8; 1];
    assert!(!buf.read_bytes_exact(6, &mut out));

    let mut out = [0u8; 4];
    assert!(!buf.read_bytes_exact(u64::MAX - 2, &mut out));
}

#[test]
fn buffer_read_bytes_exact_fails_when_get_returns_none() {
    let mut buf = Buffer::new(GetFailsAtBuffer {
        data: b"ABCDE".to_vec(),
        fail_at: 2,
    });
    let mut out = [0u8; 3];
    assert!(!buf.read_bytes_exact(0, &mut out));
}

#[test]
fn buffer_read_bytes_partial_and_empty() {
    let mut buf = buffer(b"hello".to_vec());

    let mut out = [0u8; 0];
    assert_eq!(buf.read_bytes(0, &mut out), 0);

    let mut out = [0u8; 3];
    assert_eq!(buf.read_bytes(0, &mut out), 3);
    assert_eq!(&out, b"hel");

    let mut out = [0u8; 10];
    assert_eq!(buf.read_bytes(0, &mut out), 5);
    assert_eq!(&out[..5], b"hello");

    let mut out = [0u8; 2];
    assert_eq!(buf.read_bytes(3, &mut out), 2);
    assert_eq!(&out, b"lo");

    assert_eq!(buf.read_bytes(5, &mut [0u8; 4]), 0);
    assert_eq!(buf.read_bytes(100, &mut [0u8; 4]), 0);
}

#[test]
fn buffer_read_bytes_stops_when_get_returns_none() {
    let mut buf = Buffer::new(GetFailsAtBuffer {
        data: b"ABCDE".to_vec(),
        fail_at: 2,
    });
    let mut out = [0u8; 5];
    assert_eq!(buf.read_bytes(0, &mut out), 2);
    assert_eq!(&out[..2], b"AB");
}

#[test]
fn buffer_overwrite_bytes_success_and_failure() {
    let mut buf = buffer(b"hello world".to_vec());

    assert!(buf.overwrite_bytes(0, b"HELLO"));
    assert_eq!(snapshot(&mut buf), b"HELLO world".to_vec());

    assert!(buf.overwrite_bytes(6, b"WORLD"));
    assert_eq!(snapshot(&mut buf), b"HELLO WORLD".to_vec());

    assert!(buf.overwrite_bytes(8, b"!!!"));
    assert_eq!(snapshot(&mut buf), b"HELLO WO!!!".to_vec());

    assert!(buf.overwrite_bytes(11, b""));

    assert!(!buf.overwrite_bytes(9, b"!!!"));
    assert!(!buf.overwrite_bytes(100, b"x"));
    assert!(!buf.overwrite_bytes(11, b"x"));
}

#[test]
fn buffer_overwrite_bytes_fails_when_set_fails() {
    let mut buf = Buffer::new(SetFailsAtBuffer {
        data: b"hello".to_vec(),
        fail_at: 2,
    });
    assert!(!buf.overwrite_bytes(0, b"HELLO"));
}

#[test]
fn buffer_write_bytes_without_resize() {
    let mut buf = buffer(b"abcdef".to_vec());

    assert!(buf.write_bytes(0, b"ABC"));
    assert_eq!(snapshot(&mut buf), b"ABCdef".to_vec());

    assert!(buf.write_bytes(3, b"DEF"));
    assert_eq!(snapshot(&mut buf), b"ABCDEF".to_vec());

    assert!(buf.write_bytes(6, b""));
    assert_eq!(snapshot(&mut buf), b"ABCDEF".to_vec());
}

#[test]
fn buffer_write_bytes_extends_with_resize() {
    let mut buf = buffer(b"abc".to_vec());

    assert!(buf.write_bytes(3, b"def"));
    assert_eq!(snapshot(&mut buf), b"abcdef".to_vec());

    assert!(buf.write_bytes(6, b"gh"));
    assert_eq!(snapshot(&mut buf), b"abcdefgh".to_vec());
}

#[test]
fn buffer_write_bytes_fails_when_cannot_resize() {
    let mut buf = Buffer::new(ReadOnlyBuffer {
        data: b"abc".to_vec(),
    });
    assert!(!buf.write_bytes(3, b"x"));
    assert!(!buf.write_bytes(0, b"X"));
}

#[test]
fn buffer_write_bytes_fails_when_resize_fails() {
    let mut buf = Buffer::new(ResizeFailsBuffer {
        data: b"abc".to_vec(),
    });
    assert!(buf.write_bytes(0, b"xy"));
    assert!(!buf.write_bytes(3, b"def"));
}

#[test]
fn buffer_delete_noop_and_guard_conditions() {
    let mut buf = buffer(b"hello".to_vec());

    assert!(buf.delete(0, 0));
    assert_eq!(snapshot(&mut buf), b"hello".to_vec());

    assert!(buf.delete(5, 10));
    assert_eq!(snapshot(&mut buf), b"hello".to_vec());

    assert!(buf.delete(100, 1));
    assert_eq!(snapshot(&mut buf), b"hello".to_vec());

    let mut read_only = Buffer::new(ReadOnlyBuffer {
        data: b"hello".to_vec(),
    });
    assert!(!read_only.delete(0, 1));
}

#[test]
fn buffer_delete_from_start_middle_and_end() {
    let mut buf = buffer(b"0123456789".to_vec());

    assert!(buf.delete(0, 3));
    assert_eq!(snapshot(&mut buf), b"3456789".to_vec());

    assert!(buf.delete(2, 2));
    assert_eq!(snapshot(&mut buf), b"34789".to_vec());

    assert!(buf.delete(2, 100));
    assert_eq!(snapshot(&mut buf), b"34".to_vec());
}

#[test]
fn buffer_delete_all_and_single_byte() {
    let mut buf = buffer(b"abc".to_vec());
    assert!(buf.delete(0, 3));
    assert_eq!(buf.len(), 0);

    let mut buf = buffer(b"abc".to_vec());
    assert!(buf.delete(1, 1));
    assert_eq!(snapshot(&mut buf), b"ac".to_vec());
}

#[test]
fn buffer_delete_large_buffer_crosses_internal_chunk_size() {
    let data: Vec<u8> = (0..5000u16).map(|v| (v % 256) as u8).collect();
    let mut buf = buffer(data.clone());

    assert!(buf.delete(100, 2000));
    let mut expected = data;
    expected.drain(100..2100);
    assert_eq!(snapshot(&mut buf), expected);
}

#[test]
fn buffer_insert_empty_and_invalid_position() {
    let mut buf = buffer(b"abc".to_vec());

    assert!(buf.insert(1, b""));
    assert_eq!(snapshot(&mut buf), b"abc".to_vec());

    assert!(!buf.insert(4, b"x"));
    assert!(!buf.insert(100, b"x"));

    let mut read_only = Buffer::new(ReadOnlyBuffer {
        data: b"abc".to_vec(),
    });
    assert!(!read_only.insert(0, b"x"));
}

#[test]
fn buffer_insert_at_start_middle_and_end() {
    let mut buf = buffer(b"def".to_vec());

    assert!(buf.insert(0, b"abc"));
    assert_eq!(snapshot(&mut buf), b"abcdef".to_vec());

    let mut buf = buffer(b"adef".to_vec());
    assert!(buf.insert(1, b"bc"));
    assert_eq!(snapshot(&mut buf), b"abcdef".to_vec());

    let mut buf = buffer(b"abc".to_vec());
    assert!(buf.insert(3, b"def"));
    assert_eq!(snapshot(&mut buf), b"abcdef".to_vec());
}

#[test]
fn buffer_insert_fails_when_resize_fails() {
    let mut buf = Buffer::new(ResizeFailsBuffer {
        data: b"abc".to_vec(),
    });
    assert!(!buf.insert(1, b"XY"));
}

#[test]
fn buffer_insert_large_buffer_crosses_internal_chunk_size() {
    let data: Vec<u8> = (0..5000u16).map(|v| (v % 256) as u8).collect();
    let mut buf = buffer(data.clone());
    let insert_bytes: Vec<u8> = (100..110).map(|v| (v % 256) as u8).collect();

    assert!(buf.insert(2500, &insert_bytes));
    let mut expected = data;
    for (i, b) in insert_bytes.iter().enumerate() {
        expected.insert(2500 + i, *b);
    }
    assert_eq!(snapshot(&mut buf), expected);
}

#[test]
fn buffer_vec_access_set_and_resize() {
    let mut data = vec![1u8, 2, 3];
    assert!(BufferAccess::set(&mut data, 1, 9));
    assert_eq!(data, vec![1, 9, 3]);
    assert!(!BufferAccess::set(&mut data, 3, 4));

    assert!(BufferAccess::resize(&mut data, 5));
    assert_eq!(data.len(), 5);
    assert_eq!(data[3], 0);
    assert_eq!(data[4], 0);
    assert!(BufferAccess::can_write(&data));
    assert!(BufferAccess::can_resize(&data));
}

fn hex_format(index: u32, display_chars: u32) -> ([u8; 4], u8) {
    let mut output = [b'?'; 4];
    let len = BufferView::<Vec<u8>>::hex_format(index, display_chars, &mut output);
    (output, len)
}

fn dec_format(index: u32, display_chars: u32) -> ([u8; 4], u8) {
    let mut output = [b'?'; 4];
    let len = BufferView::<Vec<u8>>::dec_format(index, display_chars, &mut output);
    (output, len)
}

fn formatted(output: &[u8; 4], len: u8) -> &[u8] {
    &output[..len as usize]
}

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

#[test]
fn hex_format_writes_two_uppercase_hex_digits() {
    let cases = [
        (0, b"00"),
        (1, b"01"),
        (9, b"09"),
        (10, b"0A"),
        (15, b"0F"),
        (16, b"10"),
        (0xAB, b"AB"),
        (0xFF, b"FF"),
        (0x1234, b"34"),
        (0x5A, b"5A"),
    ];
    for (index, expected) in cases {
        for display_chars in [0, 1, 2, 4, 8, 16] {
            let (output, len) = hex_format(index, display_chars);
            assert_eq!(len, 2, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected, "index={index}, display_chars={display_chars}");
        }
    }
}

#[test]
fn dec_format_single_digit_indices() {
    for display_chars in [1, 2, 3, 4, 8] {
        for (index, expected) in [(0, b"+0"), (5, b"+5"), (9, b"+9")] {
            let (output, len) = dec_format(index, display_chars);
            assert_eq!(len, 2, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected);
        }
    }
}

#[test]
fn dec_format_two_digit_indices_with_narrow_display() {
    let cases = [(10, b"10"), (42, b"42"), (99, b"99")];
    for display_chars in [1, 2] {
        for (index, expected) in cases {
            let (output, len) = dec_format(index, display_chars);
            assert_eq!(len, 2, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected);
        }
    }
}

#[test]
fn dec_format_two_digit_indices_with_wide_display() {
    let cases = [(10, b"+10"), (42, b"+42"), (99, b"+99")];
    for display_chars in [3, 4, 8] {
        for (index, expected) in cases {
            let (output, len) = dec_format(index, display_chars);
            assert_eq!(len, 3, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected);
        }
    }
}

#[test]
fn dec_format_three_digit_indices_with_two_display_chars() {
    let cases = [(100, b"00"), (123, b"23"), (999, b"99")];
    for (index, expected) in cases {
        let (output, len) = dec_format(index, 2);
        assert_eq!(len, 2, "index={index}");
        assert_eq!(formatted(&output, len), expected);
    }
}

#[test]
fn dec_format_three_digit_indices_with_three_display_chars() {
    let cases = [(100, b"100"), (123, b"123"), (999, b"999")];
    for (index, expected) in cases {
        let (output, len) = dec_format(index, 3);
        assert_eq!(len, 3, "index={index}");
        assert_eq!(formatted(&output, len), expected);
    }
}

#[test]
fn dec_format_three_digit_indices_with_wide_display() {
    let cases = [(100, b"+100"), (123, b"+123"), (999, b"+999")];
    for display_chars in [4, 8] {
        for (index, expected) in cases {
            let (output, len) = dec_format(index, display_chars);
            assert_eq!(len, 4, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected);
        }
    }
}

#[test]
fn dec_format_four_digit_indices_with_two_display_chars() {
    let cases = [(1000, b"00"), (1234, b"34"), (9999, b"99")];
    for (index, expected) in cases {
        let (output, len) = dec_format(index, 2);
        assert_eq!(len, 2, "index={index}");
        assert_eq!(formatted(&output, len), expected);
    }
}

#[test]
fn dec_format_four_digit_indices_with_three_display_chars() {
    let cases = [(1000, b"000"), (1234, b"234"), (9999, b"999")];
    for (index, expected) in cases {
        let (output, len) = dec_format(index, 3);
        assert_eq!(len, 3, "index={index}");
        assert_eq!(formatted(&output, len), expected);
    }
}

#[test]
fn dec_format_four_digit_indices_with_wide_display() {
    let cases = [(1000, b"1000"), (1234, b"1234"), (9999, b"9999")];
    for display_chars in [4, 8] {
        for (index, expected) in cases {
            let (output, len) = dec_format(index, display_chars);
            assert_eq!(len, 4, "index={index}, display_chars={display_chars}");
            assert_eq!(formatted(&output, len), expected);
        }
    }
}

#[test]
fn dec_format_boundary_values() {
    let (output, len) = dec_format(9, 2);
    assert_eq!(formatted(&output, len), b"+9");
    let (output, len) = dec_format(10, 2);
    assert_eq!(formatted(&output, len), b"10");
    let (output, len) = dec_format(10, 4);
    assert_eq!(formatted(&output, len), b"+10");

    let (output, len) = dec_format(99, 2);
    assert_eq!(formatted(&output, len), b"99");
    let (output, len) = dec_format(100, 2);
    assert_eq!(formatted(&output, len), b"00");
    let (output, len) = dec_format(100, 3);
    assert_eq!(formatted(&output, len), b"100");
    let (output, len) = dec_format(100, 4);
    assert_eq!(formatted(&output, len), b"+100");

    let (output, len) = dec_format(999, 3);
    assert_eq!(formatted(&output, len), b"999");
    let (output, len) = dec_format(999, 4);
    assert_eq!(formatted(&output, len), b"+999");
    let (output, len) = dec_format(1000, 2);
    assert_eq!(formatted(&output, len), b"00");
    let (output, len) = dec_format(1000, 3);
    assert_eq!(formatted(&output, len), b"000");
    let (output, len) = dec_format(1000, 4);
    assert_eq!(formatted(&output, len), b"1000");
}
