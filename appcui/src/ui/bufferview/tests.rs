use crate::graphics::{CharAttribute, Surface};
use crate::prelude::*;

use super::buffer::{Buffer, BufferAccess};
use super::bufferview::BufferView;
use super::search_parser::{parse, Error};
use super::{
    ColumnsCount, DataRepresentationFormat, Endian, Flags, FloatFormat, HexFormat, IntFormat,
    Interval, OffsetFormat, UIntFormat, Codepage,
};
use super::format::ValidateResult;
use super::output_buffer::OutputBuffer;

fn buffer(data: impl Into<Vec<u8>>) -> Buffer<Vec<u8>> {
    Buffer::new(data.into())
}

fn snapshot(buf: &mut Buffer<Vec<u8>>) -> Vec<u8> {
    let len = buf.len() as usize;
    let mut out = vec![0u8; len];
    buf.read_bytes(0, &mut out);
    out
}

#[derive(Default)]
struct ReadOnlyBuffer {
    data: Vec<u8>,
}

impl BufferAccess for ReadOnlyBuffer {
    fn count(&self) -> u64 {
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

    fn resize(&mut self, _new_size: u64, _: u8) -> bool {
        false
    }
}
#[derive(Default)]
struct ResizeFailsBuffer {
    data: Vec<u8>,
}

impl BufferAccess for ResizeFailsBuffer {
    fn count(&self) -> u64 {
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

    fn resize(&mut self, _new_size: u64, _value: u8) -> bool {
        false
    }
}
#[derive(Default)]
struct SetFailsAtBuffer {
    data: Vec<u8>,
    fail_at: u64,
}

impl BufferAccess for SetFailsAtBuffer {
    fn count(&self) -> u64 {
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

    fn resize(&mut self, new_size: u64, value: u8) -> bool {
        self.data.resize(new_size as usize, value);
        true
    }
}

#[derive(Default)]
struct GetFailsAtBuffer {
    data: Vec<u8>,
    fail_at: u64,
}

impl BufferAccess for GetFailsAtBuffer {
    fn count(&self) -> u64 {
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

    fn resize(&mut self, new_size: u64, value: u8) -> bool {
        self.data.resize(new_size as usize, value);
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

    assert!(BufferAccess::resize(&mut data, 5, 0u8));
    assert_eq!(data.len(), 5);
    assert_eq!(data[3], 0);
    assert_eq!(data[4], 0);
    assert!(BufferAccess::can_write(&data));
    assert!(BufferAccess::can_resize(&data));
}

#[test]
fn buffer_resize_grow_shrink_and_same_size() {
    let mut buf = buffer(b"hello".to_vec());

    assert!(buf.resize(8, 0xAA));
    assert_eq!(buf.len(), 8);
    assert_eq!(snapshot(&mut buf), vec![b'h', b'e', b'l', b'l', b'o', 0xAA, 0xAA, 0xAA]);

    assert!(buf.resize(3, 0xFF));
    assert_eq!(buf.len(), 3);
    assert_eq!(snapshot(&mut buf), b"hel".to_vec());

    assert!(buf.resize(3, 0x55));
    assert_eq!(snapshot(&mut buf), b"hel".to_vec());
}

#[test]
fn buffer_resize_to_empty_and_with_fill_byte() {
    let mut buf = buffer(b"abc".to_vec());

    assert!(buf.resize(0, 0xCD));
    assert_eq!(buf.len(), 0);
    assert_eq!(snapshot(&mut buf), Vec::<u8>::new());

    assert!(buf.resize(4, 0xEF));
    assert_eq!(snapshot(&mut buf), vec![0xEF, 0xEF, 0xEF, 0xEF]);
}

#[test]
fn buffer_resize_fails_when_cannot_resize() {
    let mut buf = Buffer::new(ReadOnlyBuffer {
        data: b"abc".to_vec(),
    });
    assert!(!buf.resize(10, 0));
    assert_eq!(buf.len(), 3);
}

#[test]
fn buffer_resize_fails_when_backend_resize_fails() {
    let mut buf = Buffer::new(ResizeFailsBuffer {
        data: b"abc".to_vec(),
    });
    assert!(!buf.resize(10, 0));
    assert_eq!(buf.len(), 3);
}

#[test]
fn buffer_fill_region_preserves_surrounding_bytes() {
    let mut buf = buffer(b"0123456789".to_vec());

    assert!(buf.fill(3, 4, 0xFF));
    assert_eq!(snapshot(&mut buf), vec![0x30, 0x31, 0x32, 0xFF, 0xFF, 0xFF, 0xFF, 0x37, 0x38, 0x39]);

    assert!(buf.fill(0, 2, 0xAA));
    assert_eq!(snapshot(&mut buf), vec![0xAA, 0xAA, 0x32, 0xFF, 0xFF, 0xFF, 0xFF, 0x37, 0x38, 0x39]);

    assert!(buf.fill(8, 2, 0x55));
    assert_eq!(snapshot(&mut buf), vec![0xAA, 0xAA, 0x32, 0xFF, 0xFF, 0xFF, 0xFF, 0x37, 0x55, 0x55]);
}

#[test]
fn buffer_fill_entire_buffer_and_single_byte() {
    let mut buf = buffer(b"hello".to_vec());

    assert!(buf.fill(0, 5, 0x42));
    assert_eq!(snapshot(&mut buf), vec![0x42; 5]);

    let mut buf = buffer(b"abc".to_vec());
    assert!(buf.fill(1, 1, 0x99));
    assert_eq!(snapshot(&mut buf), b"a\x99c".to_vec());
}

#[test]
fn buffer_fill_zero_count_is_noop() {
    let mut buf = buffer(b"hello".to_vec());

    assert!(buf.fill(0, 0, 0xFF));
    assert!(buf.fill(5, 0, 0xFF));
    assert!(buf.fill(100, 0, 0xFF));
    assert_eq!(snapshot(&mut buf), b"hello".to_vec());
}

#[test]
fn buffer_fill_fails_on_out_of_bounds() {
    let mut buf = buffer(b"hello".to_vec());

    assert!(!buf.fill(4, 2, 0xFF));
    assert!(!buf.fill(5, 1, 0xFF));
    assert!(!buf.fill(100, 1, 0xFF));
    assert!(!buf.fill(u64::MAX, 1, 0xFF));
    assert_eq!(snapshot(&mut buf), b"hello".to_vec());
}

#[test]
fn buffer_fill_fails_when_cannot_write() {
    let mut buf = Buffer::new(ReadOnlyBuffer {
        data: b"hello".to_vec(),
    });
    assert!(!buf.fill(0, 3, 0xFF));
    assert_eq!(buf.get(0), Some(b'h'));
    assert_eq!(buf.get(4), Some(b'o'));
}

#[test]
fn buffer_fill_fails_when_set_fails() {
    let mut buf = Buffer::new(SetFailsAtBuffer {
        data: b"0123456789".to_vec(),
        fail_at: 5,
    });
    assert!(!buf.fill(3, 5, 0xFF));
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

fn dec_offset_line(addr: u64, len: u32) -> String {
    let mut surface = Surface::new(len.max(1), 1);
    BufferView::<Vec<u8>>::write_offset(
        &mut surface,
        CharAttribute::default(),
        addr,
        len,
        0,
        OffsetFormat::Dec,
    );
    (0..len as i32)
        .filter_map(|x| surface.char(x, 0).map(|c| c.code))
        .collect()
}

#[test]
fn write_offset_dec_zero_padded() {
    assert_eq!(dec_offset_line(0, 4), "   0");
    assert_eq!(dec_offset_line(0, 1), "0");
}

#[test]
fn write_offset_dec_space_padded() {
    assert_eq!(dec_offset_line(5, 4), "   5");
    assert_eq!(dec_offset_line(42, 4), "  42");
    assert_eq!(dec_offset_line(123, 6), "   123");
}

#[test]
fn write_offset_dec_exact_width() {
    assert_eq!(dec_offset_line(999, 3), "999");
    assert_eq!(dec_offset_line(1234, 4), "1234");
}

#[test]
fn write_offset_dec_truncated_with_ellipsis() {
    assert_eq!(dec_offset_line(1234, 2), "…4");
    assert_eq!(dec_offset_line(12345, 4), "1…45");
    assert_eq!(dec_offset_line(123456, 5), "1…456");
}

#[test]
fn write_offset_dec_zero_length_is_noop() {
    let mut surface = Surface::new(4, 1);
    surface.write_string(0, 0, "----", CharAttribute::default(), false);
    BufferView::<Vec<u8>>::write_offset(
        &mut surface,
        CharAttribute::default(),
        42,
        0,
        0,
        OffsetFormat::Dec,
    );
    for x in 0..4 {
        assert_eq!(surface.char(x, 0).map(|c| c.code), Some('-'));
    }
}

#[test]
fn write_offset_dec_large_address() {
    assert_eq!(dec_offset_line(1_234_567, 8), " 1234567");
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

// --- Codepage unit tests ---

#[test]
fn codepage_builtin_names() {
    assert_eq!(Codepage::CP437.name(), "CP437");
    assert_eq!(Codepage::WINDOWS_1252.name(), "WINDOWS_1252");
    assert_eq!(Codepage::ASCII.name(), "ASCII");
}

#[test]
fn codepage_cp437_known_mappings() {
    let cp = Codepage::CP437;
    assert_eq!(cp.get(0x00), ' ');
    assert_eq!(cp.get(0x41), 'A');
    assert_eq!(cp.get(0x61), 'a');
    assert_eq!(cp.get(0xB0), '░');
    assert_eq!(cp.get(0xE0), 'α');
    assert_eq!(cp.get(0xFF), ' ');
}

#[test]
fn codepage_ascii_known_mappings() {
    let cp = Codepage::ASCII;
    assert_eq!(cp.get(0x00), ' ');
    assert_eq!(cp.get(0x09), ' ');
    assert_eq!(cp.get(0x20), ' ');
    assert_eq!(cp.get(0x41), 'A');
    assert_eq!(cp.get(0x7E), '~');
    assert_eq!(cp.get(0x7F), ' ');
    assert_eq!(cp.get(0x80), ' ');
    assert_eq!(cp.get(0xFF), ' ');
}

#[test]
fn codepage_windows_1252_known_mappings() {
    let cp = Codepage::WINDOWS_1252;
    assert_eq!(cp.get(0x41), 'A');
    assert_eq!(cp.get(0x80), '€');
    assert_eq!(cp.get(0xA9), '©');
    assert_eq!(cp.get(0xFF), 'ÿ');
}

#[test]
fn codepage_new_initializes_printable_ascii() {
    let cp = Codepage::new("Custom");
    assert_eq!(cp.name(), "Custom");
    assert_eq!(cp.get(0x00), '?');
    assert_eq!(cp.get(0x1F), '?');
    assert_eq!(cp.get(0x20), ' ');
    assert_eq!(cp.get(0x41), 'A');
    assert_eq!(cp.get(0x7E), '~');
    assert_eq!(cp.get(0x7F), '?');
    assert_eq!(cp.get(0xFF), '?');
}

#[test]
fn codepage_set_and_get() {
    let mut cp = Codepage::new("Editable");
    cp.set(0x42, 'X');
    cp.set(0xFF, '!');
    assert_eq!(cp.get(0x42), 'X');
    assert_eq!(cp.get(0xFF), '!');
    assert_eq!(cp.get(0x41), 'A');
}

#[test]
fn codepage_set_name() {
    let mut cp = Codepage::new("Old");
    cp.set_name("NewName");
    assert_eq!(cp.name(), "NewName");
}

#[test]
fn codepage_set_map_replaces_table() {
    let mut cp = Codepage::CP437;
    let mut replacement = ['?'; 256];
    replacement[0x41] = 'Z';
    cp.set_map(replacement);
    assert_eq!(cp.get(0x41), 'Z');
    assert_eq!(cp.get(0x42), '?');
    assert_eq!(cp.map(), &replacement);
}

#[test]
fn codepage_fill() {
    let mut cp = Codepage::new("Filled");
    cp.fill('.');
    for i in 0u16..256 {
        assert_eq!(cp.get(i as u8), '.', "byte 0x{i:02X} should be '.'");
    }
}

#[test]
fn codepage_map_returns_full_table() {
    let cp = Codepage::ASCII;
    assert_eq!(cp.map().len(), 256);
    assert_eq!(cp.map()[0x41], 'A');
    assert_eq!(cp.map()[0x80], ' ');
}

// --- DataRepresentationFormat unit tests ---

fn format_write(format: DataRepresentationFormat, bytes: [u8; 8]) -> Vec<u8> {
    let mut output = OutputBuffer::new();
    format.write(bytes, &mut output);
    output.as_slice().to_vec()
}

fn extend_format_bytes<const N: usize>(prefix: [u8; N]) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..N].copy_from_slice(&prefix);
    bytes
}

fn f32_le_bytes(v: f32) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(&v.to_le_bytes());
    bytes
}

fn f64_le_bytes(v: f64) -> [u8; 8] {
    v.to_le_bytes()
}

#[test]
fn data_format_hex_byte() {
    let format = DataRepresentationFormat::Hex(HexFormat::Byte);
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format.display_chars(), 2);
    assert_eq!(format_write(format, [0xAB, 0, 0, 0, 0, 0, 0, 0]), b"AB");
    assert_eq!(format.validate("A"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("AB");
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 0xAB);
}

#[test]
fn data_format_hex_word() {
    let format = DataRepresentationFormat::Hex(HexFormat::Word);
    assert_eq!(format.bytes_count(), 2);
    assert_eq!(format.display_chars(), 4);
    assert_eq!(format_write(format, [0x34, 0x12, 0, 0, 0, 0, 0, 0]), b"1234");
    let (bytes, count) = format.convert_to_bytes("1234");
    assert_eq!(count, 2);
    assert_eq!(bytes[..2], 0x1234_u16.to_le_bytes());
}

#[test]
fn data_format_hex_dword() {
    let format = DataRepresentationFormat::Hex(HexFormat::DWord);
    assert_eq!(format.bytes_count(), 4);
    assert_eq!(format.display_chars(), 8);
    assert_eq!(
        format_write(format, [0x67, 0x45, 0x23, 0x01, 0, 0, 0, 0]),
        b"01234567"
    );
    let (bytes, count) = format.convert_to_bytes("01234567");
    assert_eq!(count, 4);
    assert_eq!(bytes[..4], 0x0123_4567_u32.to_le_bytes());
}

#[test]
fn data_format_hex_qword() {
    let format = DataRepresentationFormat::Hex(HexFormat::QWord);
    assert_eq!(format.bytes_count(), 8);
    assert_eq!(format.display_chars(), 16);
    assert_eq!(
        format_write(format, [0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01]),
        b"0123456789ABCDEF"
    );
    let (bytes, count) = format.convert_to_bytes("0123456789ABCDEF");
    assert_eq!(count, 8);
    assert_eq!(bytes, 0x0123_4567_89AB_CDEF_u64.to_le_bytes());
}

#[test]
fn data_format_oct() {
    let format = DataRepresentationFormat::Oct;
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format.display_chars(), 3);
    assert_eq!(format_write(format, [0xFF, 0, 0, 0, 0, 0, 0, 0]), b"377");
    assert_eq!(format.validate("37"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("377");
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 0xFF);
}

#[test]
fn data_format_bin() {
    let format = DataRepresentationFormat::Bin;
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format.display_chars(), 8);
    assert_eq!(format_write(format, [0xAB, 0, 0, 0, 0, 0, 0, 0]), b"10101011");
    assert_eq!(format.validate("101"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("10101011");
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 0xAB);
}

#[test]
fn data_format_uint_u8() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U8);
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format_write(format, [0xFF, 0, 0, 0, 0, 0, 0, 0]), b"255");
    let (bytes, count) = format.convert_to_bytes("255");
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 255);
}

#[test]
fn data_format_uint_u16() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U16);
    assert_eq!(format.bytes_count(), 2);
    assert_eq!(
        format_write(format, extend_format_bytes([0xFF, 0xFF])),
        b"65535"
    );
    let (bytes, count) = format.convert_to_bytes("65535");
    assert_eq!(count, 2);
    assert_eq!(bytes[..2], 65535_u16.to_le_bytes());
}

#[test]
fn data_format_uint_u32() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U32);
    assert_eq!(format.bytes_count(), 4);
    assert_eq!(
        format_write(format, extend_format_bytes(u32::MAX.to_le_bytes())),
        b"4294967295"
    );
    let (bytes, count) = format.convert_to_bytes("1000000");
    assert_eq!(count, 4);
    assert_eq!(bytes[..4], 1_000_000_u32.to_le_bytes());
}

#[test]
fn data_format_uint_u64() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U64);
    assert_eq!(format.bytes_count(), 8);
    assert_eq!(
        format_write(format, u64::MAX.to_le_bytes()),
        b"18446744073709551615"
    );
    let (bytes, count) = format.convert_to_bytes("42");
    assert_eq!(count, 8);
    assert_eq!(bytes, 42_u64.to_le_bytes());
}

#[test]
fn data_format_int_i8() {
    let format = DataRepresentationFormat::Int(IntFormat::I8);
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format_write(format, [0x80, 0, 0, 0, 0, 0, 0, 0]), b"-128");
    let (bytes, count) = format.convert_to_bytes("-128");
    assert_eq!(count, 1);
    assert_eq!(bytes[0] as i8, -128);
}

#[test]
fn data_format_int_i16() {
    let format = DataRepresentationFormat::Int(IntFormat::I16);
    assert_eq!(format.bytes_count(), 2);
    assert_eq!(
        format_write(format, extend_format_bytes([0x00, 0x80])),
        b"-32768"
    );
    let (bytes, count) = format.convert_to_bytes("256");
    assert_eq!(count, 2);
    assert_eq!(bytes[..2], 256_i16.to_le_bytes());
}

#[test]
fn data_format_int_i32() {
    let format = DataRepresentationFormat::Int(IntFormat::I32);
    assert_eq!(format.bytes_count(), 4);
    assert_eq!(
        format_write(format, extend_format_bytes(i32::MIN.to_le_bytes())),
        b"-2147483648"
    );
    let (bytes, count) = format.convert_to_bytes("1000");
    assert_eq!(count, 4);
    assert_eq!(bytes[..4], 1000_i32.to_le_bytes());
}

#[test]
fn data_format_int_i64() {
    let format = DataRepresentationFormat::Int(IntFormat::I64);
    assert_eq!(format.bytes_count(), 8);
    assert_eq!(
        format_write(format, 1_000_000_i64.to_le_bytes()),
        b"            +1000000"
    );
    let (bytes, count) = format.convert_to_bytes("-1");
    assert_eq!(count, 8);
    assert_eq!(bytes, (-1_i64).to_le_bytes());
}

#[test]
fn data_format_float_scientific32() {
    let format = DataRepresentationFormat::Float(FloatFormat::Scientific32);
    assert_eq!(format.bytes_count(), 4);
    assert_eq!(format.display_chars(), 9);
    assert_eq!(format_write(format, f32_le_bytes(1.0)), b"+1.00e+00");
    assert_eq!(format.validate("1.5"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("1.5");
    assert_eq!(count, 4);
    assert_eq!(bytes[..4], 1.5_f32.to_le_bytes());
}

#[test]
fn data_format_float_scientific64() {
    let format = DataRepresentationFormat::Float(FloatFormat::Scientific64);
    assert_eq!(format.bytes_count(), 8);
    assert_eq!(format.display_chars(), 23);
    assert_eq!(
        format_write(format, f64_le_bytes(-2.5)),
        b"-2.500000000000000e+000"
    );
    let (bytes, count) = format.convert_to_bytes("-2.5");
    assert_eq!(count, 8);
    assert_eq!(bytes, (-2.5_f64).to_le_bytes());
}

#[test]
fn data_format_float_e4m3() {
    let format = DataRepresentationFormat::Float(FloatFormat::E4M3);
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format_write(format, [0x77, 0, 0, 0, 0, 0, 0, 0]), b"+240.000");
    assert_eq!(format.validate("+240.000"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("+240.000");
    assert_eq!(count, 1);
    assert_eq!(bytes[0], 0x77);
}

#[test]
fn data_format_float_e5m2() {
    let format = DataRepresentationFormat::Float(FloatFormat::E5M2);
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format_write(format, [0x7C, 0, 0, 0, 0, 0, 0, 0]), b"          inf");
    assert_eq!(format.validate("inf"), ValidateResult::Valid);
    let (bytes, count) = format.convert_to_bytes("+57344.000000");
    assert_eq!(count, 1);
    assert_eq!(format_write(format, bytes), b"+57344.000000");
}

#[test]
fn data_format_char() {
    let format = DataRepresentationFormat::Char;
    assert!(format.is_char());
    assert_eq!(format.bytes_count(), 1);
    assert_eq!(format.display_chars(), 1);
    assert_eq!(format_write(format, [b'Z', 0, 0, 0, 0, 0, 0, 0]), b"Z");
    assert_eq!(format.validate("A"), ValidateResult::Update);
    let (bytes, count) = format.convert_to_bytes("Hi");
    assert_eq!(count, 2);
    assert_eq!(&bytes[..2], b"Hi");
}

// --- DataRepresentationFormat::validate unit tests ---

#[test]
fn data_format_validate_empty_for_all_formats() {
    let formats = [
        DataRepresentationFormat::Hex(HexFormat::Byte),
        DataRepresentationFormat::Hex(HexFormat::Word),
        DataRepresentationFormat::Hex(HexFormat::DWord),
        DataRepresentationFormat::Hex(HexFormat::QWord),
        DataRepresentationFormat::Oct,
        DataRepresentationFormat::Bin,
        DataRepresentationFormat::UInt(UIntFormat::U8),
        DataRepresentationFormat::UInt(UIntFormat::U16),
        DataRepresentationFormat::UInt(UIntFormat::U32),
        DataRepresentationFormat::UInt(UIntFormat::U64),
        DataRepresentationFormat::Int(IntFormat::I8),
        DataRepresentationFormat::Int(IntFormat::I16),
        DataRepresentationFormat::Int(IntFormat::I32),
        DataRepresentationFormat::Int(IntFormat::I64),
        DataRepresentationFormat::Float(FloatFormat::Scientific32),
        DataRepresentationFormat::Float(FloatFormat::Scientific64),
        DataRepresentationFormat::Float(FloatFormat::E4M3),
        DataRepresentationFormat::Float(FloatFormat::E5M2),
        DataRepresentationFormat::Char,
    ];
    for format in formats {
        assert_eq!(format.validate(""), ValidateResult::Valid);
    }
}

#[test]
fn data_format_validate_hex_byte() {
    let format = DataRepresentationFormat::Hex(HexFormat::Byte);
    assert_eq!(format.validate("A"), ValidateResult::Valid);
    assert_eq!(format.validate("a"), ValidateResult::Valid);
    assert_eq!(format.validate("AB"), ValidateResult::Update);
    assert_eq!(format.validate("ab"), ValidateResult::Update);
    assert_eq!(format.validate("G"), ValidateResult::FormatError);
    assert_eq!(format.validate("1Z"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_hex_word() {
    let format = DataRepresentationFormat::Hex(HexFormat::Word);
    assert_eq!(format.validate("123"), ValidateResult::Valid);
    assert_eq!(format.validate("1234"), ValidateResult::Update);
    assert_eq!(format.validate("12G4"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_hex_dword() {
    let format = DataRepresentationFormat::Hex(HexFormat::DWord);
    assert_eq!(format.validate("0123456"), ValidateResult::Valid);
    assert_eq!(format.validate("01234567"), ValidateResult::Update);
    assert_eq!(format.validate("0123456G"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_hex_qword() {
    let format = DataRepresentationFormat::Hex(HexFormat::QWord);
    assert_eq!(format.validate("0123456789ABCDE"), ValidateResult::Valid);
    assert_eq!(format.validate("0123456789ABCDEF"), ValidateResult::Update);
    assert_eq!(format.validate("0123456789ABCDEG"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_oct() {
    let format = DataRepresentationFormat::Oct;
    assert_eq!(format.validate("25"), ValidateResult::Valid);
    assert_eq!(format.validate("377"), ValidateResult::Update);
    assert_eq!(format.validate("28"), ValidateResult::FormatError);
    assert_eq!(format.validate("9"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_bin() {
    let format = DataRepresentationFormat::Bin;
    assert_eq!(format.validate("1010101"), ValidateResult::Valid);
    assert_eq!(format.validate("10101011"), ValidateResult::Update);
    assert_eq!(format.validate("10101012"), ValidateResult::FormatError);
    assert_eq!(format.validate("abcdefgh"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_uint_u8() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U8);
    assert_eq!(format.validate("25"), ValidateResult::Valid);
    assert_eq!(format.validate("255"), ValidateResult::Valid);
    assert_eq!(format.validate(" 42 "), ValidateResult::Valid);
    assert_eq!(format.validate("-1"), ValidateResult::FormatError);
    assert_eq!(format.validate("12a"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_uint_u16() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U16);
    assert_eq!(format.validate("6553"), ValidateResult::Valid);
    assert_eq!(format.validate("65535"), ValidateResult::Valid);
    assert_eq!(format.validate("+42"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_uint_u32() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U32);
    assert_eq!(format.validate("1000000"), ValidateResult::Valid);
    assert_eq!(format.validate("  99  "), ValidateResult::Valid);
    assert_eq!(format.validate("1.5"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_uint_u64() {
    let format = DataRepresentationFormat::UInt(UIntFormat::U64);
    assert_eq!(format.validate("1844674407370955161"), ValidateResult::Valid);
    assert_eq!(format.validate("0"), ValidateResult::Valid);
    assert_eq!(format.validate("abc"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_int_i8() {
    let format = DataRepresentationFormat::Int(IntFormat::I8);
    assert_eq!(format.validate("+7"), ValidateResult::Valid);
    assert_eq!(format.validate("-128"), ValidateResult::Valid);
    assert_eq!(format.validate("  99 "), ValidateResult::Valid);
    assert_eq!(format.validate("12a"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_int_i16() {
    let format = DataRepresentationFormat::Int(IntFormat::I16);
    assert_eq!(format.validate("256"), ValidateResult::Valid);
    assert_eq!(format.validate("-32768"), ValidateResult::Valid);
    assert_eq!(format.validate("1.5"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_int_i32() {
    let format = DataRepresentationFormat::Int(IntFormat::I32);
    assert_eq!(format.validate("123"), ValidateResult::Valid);
    assert_eq!(format.validate("-42"), ValidateResult::Valid);
    assert_eq!(format.validate("++1"), ValidateResult::Valid);
    assert_eq!(format.validate("12a"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_int_i64() {
    let format = DataRepresentationFormat::Int(IntFormat::I64);
    assert_eq!(format.validate("1000000"), ValidateResult::Valid);
    assert_eq!(format.validate("-1"), ValidateResult::Valid);
    assert_eq!(format.validate("1e3"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_float_scientific32() {
    let format = DataRepresentationFormat::Float(FloatFormat::Scientific32);
    assert_eq!(format.validate("1.2"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2e3"), ValidateResult::Valid);
    assert_eq!(format.validate("-inf"), ValidateResult::Valid);
    assert_eq!(format.validate("nan"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2.3"), ValidateResult::FormatError);
    assert_eq!(format.validate("abc"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_float_scientific64() {
    let format = DataRepresentationFormat::Float(FloatFormat::Scientific64);
    assert_eq!(format.validate("-2.5"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2e+03"), ValidateResult::Valid);
    assert_eq!(format.validate("NaN"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2.3"), ValidateResult::FormatError);
    assert_eq!(format.validate("xyz"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_float_e4m3() {
    let format = DataRepresentationFormat::Float(FloatFormat::E4M3);
    assert_eq!(format.validate("+240.000"), ValidateResult::Valid);
    assert_eq!(format.validate("-0.5"), ValidateResult::Valid);
    assert_eq!(format.validate("NaN"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2e3"), ValidateResult::FormatError);
    assert_eq!(format.validate("1..0"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_float_e5m2() {
    let format = DataRepresentationFormat::Float(FloatFormat::E5M2);
    assert_eq!(format.validate("inf"), ValidateResult::Valid);
    assert_eq!(format.validate("+57344.000000"), ValidateResult::Valid);
    assert_eq!(format.validate("-inf"), ValidateResult::Valid);
    assert_eq!(format.validate("1.2e3"), ValidateResult::FormatError);
    assert_eq!(format.validate("not-a-number"), ValidateResult::FormatError);
}

#[test]
fn data_format_validate_char() {
    let format = DataRepresentationFormat::Char;
    assert_eq!(format.validate("A"), ValidateResult::Update);
    assert_eq!(format.validate("Hi"), ValidateResult::Update);
    assert_eq!(format.validate(" "), ValidateResult::Update);
}

// --- BufferView unit tests ---

fn test_buffer_data() -> Vec<u8> {
    let mut data: Vec<u8> = (0..32).collect();
    data.extend_from_slice(b"Hello!");
    data
}

fn test_large_buffer_data() -> Vec<u8> {
    (0u32..200_001).map(|i| (i % 256) as u8).collect()
}

fn make_bufferview(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress | Flags::SearchBar,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Hex(HexFormat::Byte));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_for_mouse(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Hex(HexFormat::Byte));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_with_intervals(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = make_bufferview(data);
    bv.set_interval_names_visible(true);
    bv.set_intervals(&[
        Interval::new(0, 16, CharAttribute::with_fore_color(Color::Red), "Header"),
        Interval::new(16, 16, CharAttribute::with_fore_color(Color::Green), "Body"),
    ]);
    bv
}

fn make_bufferview_char_mode(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(16));
    bv.set_data_representation_format(DataRepresentationFormat::Char);
    bv
}

fn make_bufferview_oct_auto(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Auto);
    bv.set_data_representation_format(DataRepresentationFormat::Oct);
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_char_auto(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Auto);
    bv.set_data_representation_format(DataRepresentationFormat::Char);
    bv
}

fn make_bufferview_int_i8(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress | Flags::SearchBar,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Int(IntFormat::I8));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_uint_u8(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::UInt(UIntFormat::U8));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_int_edit_i8(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Int(IntFormat::I8));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_float_e4m3(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Float(FloatFormat::E4M3));
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_bin(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Bin);
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

fn make_bufferview_oct(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress,
    );
    bv.set_columns_count(ColumnsCount::Fixed(8));
    bv.set_data_representation_format(DataRepresentationFormat::Oct);
    bv.set_offset_format(OffsetFormat::Hex);
    bv
}

#[test]
fn bufferview_create_defaults() {
    let bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress | Flags::SearchBar,
    );
    assert_eq!(bv.bytes_count(), 38);
    assert_eq!(bv.current_pos(), 0);
    assert!(!bv.has_selection());
    assert!(bv.is_address_visible());
    assert!(!bv.is_interval_names_visible());
    assert!(!bv.is_ascii_strings_visible());
    assert!(!bv.is_utf16_ascii_strings_visible());
    assert!(matches!(bv.offset_format(), OffsetFormat::Hex));
    assert!(matches!(bv.endian(), Endian::Little));
    assert!(matches!(bv.format(), DataRepresentationFormat::Hex(HexFormat::Byte)));
}

#[test]
fn bufferview_selection_api() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::from_value(0).expect("empty flags"),
    );
    assert!(bv.set_selection(2, 6));
    assert!(bv.has_selection());
    assert_eq!(bv.selection(), Some((2, 6)));
    bv.clear_selection();
    assert!(!bv.has_selection());
    assert_eq!(bv.selection(), None);
}

#[test]
fn bufferview_set_selection_valid_ranges() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::from_value(0).expect("empty flags"),
    );
    let len = bv.bytes_count();

    assert!(bv.set_selection(0, 1));
    assert_eq!(bv.selection(), Some((0, 1)));

    assert!(bv.set_selection(2, 6));
    assert_eq!(bv.selection(), Some((2, 6)));

    assert!(bv.set_selection(37, len));
    assert_eq!(bv.selection(), Some((37, len)));

    assert!(bv.set_selection(0, len));
    assert_eq!(bv.selection(), Some((0, len)));
}

#[test]
fn bufferview_set_selection_rejects_invalid_ranges() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::from_value(0).expect("empty flags"),
    );
    let len = bv.bytes_count();

    assert!(bv.set_selection(4, 10));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(4, 4));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(10, 4));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(0, len + 1));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(len, len));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(len - 1, len + 1));
    assert_eq!(bv.selection(), Some((4, 10)));

    assert!(!bv.set_selection(100, 200));
    assert_eq!(bv.selection(), Some((4, 10)));
}

#[test]
fn bufferview_default_codepage_endian_and_format() {
    let bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    assert_eq!(bv.codepage().name(), "CP437");
    assert!(matches!(bv.endian(), Endian::Little));
    assert!(matches!(
        bv.data_representation_format(),
        DataRepresentationFormat::Hex(HexFormat::Byte)
    ));
    assert!(matches!(bv.format(), DataRepresentationFormat::Hex(HexFormat::Byte)));
}

#[test]
fn bufferview_address_visible_noop_when_unchanged() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    assert!(bv.is_address_visible());
    bv.set_address_visible(true);
    assert!(bv.is_address_visible());

    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::SearchBar,
    );
    assert!(!bv.is_address_visible());
    bv.set_address_visible(false);
    assert!(!bv.is_address_visible());
}

fn test_buffer_high_bytes() -> Vec<u8> {
    vec![0x00, 0x7F, 0x80, 0xFF, 0xB0, 0xE0, 0x12, 0x34]
}

#[test]
fn bufferview_set_address_name() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    bv.set_address_name("Offset");
    bv.set_address_name("Memory");
}

#[test]
fn bufferview_set_address_width_while_hidden() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::SearchBar,
    );
    bv.set_address_width(100);
    bv.set_address_width(0);
    bv.set_address_name("Addr");
}

#[test]
fn bufferview_set_interval_name_title() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    bv.set_interval_name_title("Section");
    bv.set_interval_name_title("Region");
}

#[test]
fn bufferview_set_interval_name_width_while_hidden() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    assert!(!bv.is_interval_names_visible());
    bv.set_interval_name_width(100);
    bv.set_interval_name_width(0);
}

#[test]
fn bufferview_ascii_strings_visible_noop_when_unchanged() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    assert!(!bv.is_ascii_strings_visible());
    bv.set_ascii_strings_visible(false);
    assert!(!bv.is_ascii_strings_visible());
}

#[test]
fn bufferview_utf16_ascii_strings_visible_noop_when_unchanged() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    assert!(!bv.is_utf16_ascii_strings_visible());
    bv.set_utf16_ascii_strings_visible(false);
    assert!(!bv.is_utf16_ascii_strings_visible());
}

#[test]
fn bufferview_decode_utf8_noop_when_unchanged() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ShowAddress,
    );
    bv.set_decode_utf8(false);
    bv.set_decode_utf8(false);
}

#[test]
fn bufferview_interval_name_at() {
    let _a = App::debug(60, 15, "Paint.Enable(false)").build().unwrap();
    let bv = make_bufferview_with_intervals(test_buffer_data());
    assert_eq!(bv.interval_name_at(0), Some("Header"));
    assert_eq!(bv.interval_name_at(15), Some("Header"));
    assert_eq!(bv.interval_name_at(16), Some("Body"));
    assert_eq!(bv.interval_name_at(31), Some("Body"));
    assert_eq!(bv.interval_name_at(32), None);
    assert_eq!(bv.interval_name_at(100), None);
}

fn test_buffer_ascii_strings() -> Vec<u8> {
    let mut data = vec![0u8; 8];
    data.extend_from_slice(b"HelloWorld!!!!");
    data
}

fn test_buffer_utf16_ascii_strings() -> Vec<u8> {
    let mut data = vec![0u8; 4];
    for c in "Hello!!".chars() {
        data.push(c as u8);
        data.push(0);
    }
    data
}

fn test_buffer_utf8_chars() -> Vec<u8> {
    vec![b'H', 0xC3, 0xA9, b'l', b'l', b'o', 0x00, 0x00, 0x00, 0x00]
}

fn test_buffer_char_mode_mixed() -> Vec<u8> {
    let mut data = vec![0u8; 4];
    data.extend_from_slice(b"HelloWorld");
    data.extend_from_slice(&[0xFF, 0xFF]);
    for c in "Tests!!!!".chars() {
        data.push(c as u8);
        data.push(0);
    }
    data.push(0xFF);
    data.extend_from_slice(&[0xC3, 0xA9, b'|', 0xE2, 0x82, 0xAC, b'|', 0xF0, 0x9F, 0x98, 0x80]);
    data.extend_from_slice(&[0u8; 6]);
    data
}

fn test_buffer_char_mode_large() -> Vec<u8> {
    let mut data: Vec<u8> = (0u8..200).collect();
    data.extend_from_slice(b"HelloWorldASCII!!");
    for c in "Hello".chars() {
        data.push(c as u8);
        data.push(0);
    }
    data.extend_from_slice(&[0, 0]);
    for c in "Tests!!!!".chars() {
        data.push(c as u8);
        data.push(0);
    }
    data.extend_from_slice(&[0xC3, 0xA9, b'|', 0xE2, 0x82, 0xAC, b'|', 0xF0, 0x9F, 0x98, 0x80]);
    while data.len() <= 1024 {
        data.extend_from_slice(b"PAD");
    }
    assert!(data.len() > 1024);
    data
}

fn make_bufferview_char_mode_features(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = BufferView::with_buffer(
        data,
        layout!("d:f"),
        Flags::ScrollBars,
    );
    bv.set_columns_count(ColumnsCount::Fixed(32));
    bv.set_data_representation_format(DataRepresentationFormat::Char);
    bv.set_intervals(&[
        Interval::new(0, 256, CharAttribute::with_fore_color(Color::Red), "Header"),
        Interval::new(256, 512, CharAttribute::with_fore_color(Color::Green), "Unicode"),
        Interval::new(768, 256, CharAttribute::with_fore_color(Color::Yellow), "Tail"),
    ]);
    bv
}

fn make_bufferview_char_mode_all_decodings(data: Vec<u8>) -> BufferView<Vec<u8>> {
    let mut bv = make_bufferview_char_mode(data);
    bv.set_ascii_strings_visible(true);
    bv.set_utf16_ascii_strings_visible(true);
    bv.set_decode_utf8(true);
    bv
}

#[test]
fn test_buffer_char_mode_mixed_contains_utf8_sequences() {
    let data = test_buffer_char_mode_mixed();
    assert!(data.starts_with(&[0, 0, 0, 0]));
    assert_eq!(&data[4..14], b"HelloWorld");

    let utf16_start = 16usize;
    let mut pos = utf16_start;
    for c in "Tests!!!!".chars() {
        assert_eq!(data[pos], c as u8);
        assert_eq!(data[pos + 1], 0);
        pos += 2;
    }

    let two_byte = &data[35..37];
    let three_byte = &data[38..41];
    let four_byte = &data[42..46];
    assert_eq!(two_byte, [0xC3, 0xA9]);
    assert_eq!(three_byte, [0xE2, 0x82, 0xAC]);
    assert_eq!(four_byte, [0xF0, 0x9F, 0x98, 0x80]);
    assert_eq!(std::str::from_utf8(two_byte).unwrap(), "é");
    assert_eq!(std::str::from_utf8(three_byte).unwrap(), "€");
    assert_eq!(std::str::from_utf8(four_byte).unwrap(), "😀");
}

fn decode_utf8_from(data: &[u8], pos: u64, b: u8) -> Option<(char, u8)> {
    let mut bv = BufferView::with_buffer(
        data.to_vec(),
        layout!("d:f"),
        Flags::from_value(0).expect("empty flags"),
    );
    bv.decode_utf8(pos, b)
}

#[test]
fn decode_utf8_two_byte_sequence() {
    let data = [0xC3, 0xA9, b'x'];
    assert_eq!(decode_utf8_from(&data, 0, data[0]), Some(('é', 2)));
}

#[test]
fn decode_utf8_three_byte_sequence() {
    let data = [0xE2, 0x82, 0xAC, b'x'];
    assert_eq!(decode_utf8_from(&data, 0, data[0]), Some(('€', 3)));
}

#[test]
fn decode_utf8_four_byte_sequence() {
    let data = [0xF0, 0x9F, 0x98, 0x80, b'x'];
    assert_eq!(decode_utf8_from(&data, 0, data[0]), Some(('😀', 4)));
}

#[test]
fn decode_utf8_invalid_lead_byte() {
    let data = [0xFF, 0x00];
    assert_eq!(decode_utf8_from(&data, 0, data[0]), None);
}

#[test]
fn bufferview_read_only_blocks_edits() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ReadOnly,
    );
    assert!(!bv.delete_bytes(0, 1));
    assert!(!bv.insert_bytes(0, b"x"));
    assert!(!bv.overwrite_bytes(0, b"x"));
    assert!(!bv.resize_buffer(10, 0));
    assert!(!bv.fill_buffer(0, 1, 0));
    assert_eq!(bv.read_bytes(0, &mut [0u8; 1]), 0);
}

#[test]
fn bufferview_read_bytes_into_vec() {
    let mut bv = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::from_value(0).expect("empty flags"),
    );

    let mut output = vec![0xAA, 0xBB];
    bv.read_bytes_into_vec(32, 6, &mut output);
    assert_eq!(output, vec![0xAA, 0xBB, b'H', b'e', b'l', b'l', b'o', b'!']);

    let mut partial = Vec::new();
    bv.read_bytes_into_vec(35, 10, &mut partial);
    assert_eq!(partial, b"lo!");

    let mut bv_ro = BufferView::with_buffer(
        test_buffer_data(),
        layout!("d:f"),
        Flags::ReadOnly,
    );
    let mut read_only_output = vec![1, 2, 3];
    bv_ro.read_bytes_into_vec(0, 4, &mut read_only_output);
    assert_eq!(read_only_output, vec![1, 2, 3]);
}

#[test]
fn check_bufferview_resize_buffer_with_commands() {
    const SHRINK_SIZE: u64 = 20;
    const GROW_STEP: u64 = 8;
    const FILL_BYTE: u8 = 0xFE;

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x54588344E8C41577)
        Key.Pressed(End)
        Paint('2. Cursor at end')
        CheckHash(0x4465F6C039F70697)
        Key.Pressed(F3)
        Paint('3. Resize smaller')
        CheckHash(0x33E7D9CFB22E16E8)
        Key.Pressed(F4)
        Paint('4. Resize bigger once')
        CheckHash(0x192587547D66F859)
        Key.Pressed(F4)
        Paint('5. Resize bigger twice')
        CheckHash(0x58A114604C574F67)
    ";

    #[Window(events=CommandBarEvents,commands:A+B, internal:true)]
    struct MyWin {
        buffer_handle: Handle<BufferView<Vec<u8>>>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:60,h:12,flags: Sizeable"),
                buffer_handle: Handle::None,
            };
            w.buffer_handle = w.add(make_bufferview_for_mouse(test_buffer_data()));
            w
        }

        fn resize_smaller(&mut self) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                bv.resize_buffer(SHRINK_SIZE, 0);
            }
        }

        fn resize_bigger(&mut self) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                let new_size = bv.bytes_count() + GROW_STEP;
                bv.resize_buffer(new_size, FILL_BYTE);
            }
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F3"), "Resize Small", mywin::Commands::A);
            commandbar.set(key!("F4"), "Resize Big", mywin::Commands::B);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.resize_smaller(),
                mywin::Commands::B => self.resize_bigger(),
            }
        }
    }

    let mut a = App::debug(60, 15, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_bufferview_insert_and_overwrite_bytes_with_commands() {
    const INSERT_BYTES: &[u8] = b"ABC";
    const OVERWRITE_BYTES: &[u8] = b"XY";

    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x76270CE76FCBA0DC)
        Key.Pressed(End)
        Paint('2. Cursor at end')
        CheckHash(0xB79FD1E5D3F89AFC)
        Key.Pressed(F3)
        Paint('3. Insert bytes')
        CheckHash(0x6A4AB3D62B9B8ED1)
        Key.Pressed(F4)
        Paint('4. Overwrite bytes')
        CheckHash(0xFA4F3E2E8D8F1F85)
    ";

    #[Window(events=CommandBarEvents,commands:A+B, internal:true)]
    struct MyWin {
        buffer_handle: Handle<BufferView<Vec<u8>>>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:60,h:12,flags: Sizeable"),
                buffer_handle: Handle::None,
            };
            w.buffer_handle = w.add(make_bufferview_for_mouse(test_buffer_data()));
            w
        }

        fn insert_at_cursor(&mut self) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                let pos = bv.current_pos();
                bv.insert_bytes(pos, INSERT_BYTES);
            }
        }

        fn overwrite_at_cursor(&mut self) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                let pos = bv.current_pos();
                bv.overwrite_bytes(pos, OVERWRITE_BYTES);
            }
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F3"), "Insert Bytes", mywin::Commands::A);
            commandbar.set(key!("F4"), "Overwrite Bytes", mywin::Commands::B);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.insert_at_cursor(),
                mywin::Commands::B => self.overwrite_at_cursor(),
            }
        }
    }

    let mut a = App::debug(60, 15, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}

#[test]
fn check_bufferview_bin_edit_with_tab_and_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6D1403D1B65FE07F)
        Key.TypeText('10101010')
        Paint('2. First byte fully written')
        CheckHash(0x81D9CF40AEBAA9AF)
        Key.TypeText('1111')
        Key.Pressed(Tab)
        Paint('3. Second byte committed with Tab')
        CheckHash(0xDC7F7AC34C9DA3E6)
        Key.TypeText('0011')
        Key.Pressed(Right)
        Paint('4. Third byte committed with Right')
        CheckHash(0x83EE8CC0DBECCCCF)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_bin(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_oct_edit_with_tab_and_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x48418EB0061457AC)
        Key.TypeText('377')
        Paint('2. First byte fully written')
        CheckHash(0x975FEBE0425ADDA7)
        Key.TypeText('12')
        Key.Pressed(Tab)
        Paint('3. Second byte committed with Tab')
        CheckHash(0xF526FA131AE10C4F)
        Key.TypeText('7')
        Key.Pressed(Right)
        Paint('4. Third byte committed with Right')
        CheckHash(0x9303DEA38DA4498D)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_oct(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_uint_edit_with_tab_enter_and_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x67C5151D18D655E2)
        Key.TypeText('255')
        Key.Pressed(Tab)
        Paint('2. First byte committed with Tab')
        CheckHash(0x7F83D904B7084BB8)
        Key.TypeText('42')
        Key.Pressed(Enter)
        Paint('3. Second byte committed with Enter')
        CheckHash(0x9BAE79A8E449BF65)
        Key.TypeText('7')
        Key.Pressed(Right)
        Paint('4. Third byte committed with Right')
        CheckHash(0xA05BBB3ABA4EE57B)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_uint_u8(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_int_edit_with_tab_enter_and_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xC31C67AA0F903E4E)
        Key.TypeText('-1')
        Key.Pressed(Tab)
        Paint('2. First byte committed with Tab')
        CheckHash(0x21403C761E4D6061)
        Key.TypeText('12')
        Key.Pressed(Enter)
        Paint('3. Second byte committed with Enter')
        CheckHash(0xE0F2C458B15A4FA9)
        Key.TypeText('-7')
        Key.Pressed(Right)
        Paint('4. Third byte committed with Right')
        CheckHash(0x7CAADAA48536E9B8)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_int_edit_i8(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_float_edit_with_tab_enter_and_right() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x14322C23B27E27A7)
        Key.TypeText('+240.000')
        Key.Pressed(Tab)
        Paint('2. First byte committed with Tab')
        CheckHash(0x62108A34EE1C9B25)
        Key.TypeText('+0.500')
        Key.Pressed(Enter)
        Paint('3. Second byte committed with Enter')
        CheckHash(0x7FD13B93098BD4DE)
        Key.TypeText('+1.500')
        Key.Pressed(Right)
        Paint('4. Third byte committed with Right')
        CheckHash(0xEA681B51D283EA4E)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_float_e4m3(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_ascii_codepage() {
    let script = "
        Paint.Enable(false)
        Paint('ASCII codepage')
        CheckHash(0xEB04DE43EB28189B)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = BufferView::with_buffer(
        test_buffer_high_bytes(),
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress | Flags::SearchBar,
    );
    bv.set_columns_count(ColumnsCount::Fixed(4));
    bv.set_codepage(Codepage::ASCII);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_big_endian_uint16() {
    let script = "
        Paint.Enable(false)
        Paint('Big endian u16')
        CheckHash(0x2B8EDFBE0E457FEC)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = BufferView::with_buffer(
        test_buffer_high_bytes(),
        layout!("d:f"),
        Flags::ScrollBars | Flags::ShowAddress | Flags::SearchBar,
    );
    bv.set_columns_count(ColumnsCount::Fixed(4));
    bv.set_data_representation_format(DataRepresentationFormat::UInt(UIntFormat::U16));
    bv.set_endian(Endian::Big);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_custom_address_name() {
    let script = "
        Paint.Enable(false)
        Paint('Custom address title')
        CheckHash(0x212D5CD189C85A12)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview(test_buffer_data());
    bv.set_address_name("Offset");
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_wide_address_column() {
    let script = "
        Paint.Enable(false)
        Paint('Wide address column')
        CheckHash(0xD78FF8C6527DD441)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview(test_buffer_data());
    bv.set_address_width(14);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_address_hidden() {
    let script = "
        Paint.Enable(false)
        Paint('Address column hidden')
        CheckHash(0x7AF587C34BAFFFBA)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview(test_buffer_data());
    bv.set_address_visible(false);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_create() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x6F386642B4BD3242)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_create_with_macro() {
    let script = "
        Paint.Enable(false)
        Paint('Initial state')
        CheckHash(0x6F386642B4BD3242)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = bufferview!("Vec<u8>,d:f,flags:ScrollBars+ShowAddress+SearchBar,columns:8,format:Hex,offset:Hex");
    bv.set_buffer(test_buffer_data());
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_disabled_ignores_key_and_mouse() {
    let script = "
        Paint.Enable(false)
        Paint('1. Disabled initial state')
        CheckHash(0x7C4CBB97ED2BFF63)
        Key.Pressed(Right,3)
        Key.Pressed(End)
        Paint('2. Keys ignored')
        CheckHash(0x7C4CBB97ED2BFF63)
        Mouse.Click(20,5,left)
        Mouse.Wheel(20,5,down,2)
        Paint('3. Mouse ignored')
        CheckHash(0x7C4CBB97ED2BFF63)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview_for_mouse(test_buffer_data());
    bv.set_enabled(false);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_int_i8_large_buffer_navigation_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF99606F54DC84450)
        Key.Pressed(Right)
        Paint('2. Right')
        CheckHash(0x83154247E8246C30)
        Key.Pressed(Down)
        Paint('3. Down')
        CheckHash(0x051859861A698854)
        Key.Pressed(Left)
        Paint('4. Left')
        CheckHash(0x7A00479BACE50790)
        Key.Pressed(Up)
        Paint('5. Up')
        CheckHash(0xF99606F54DC84450)
        Key.Pressed(PageDown)
        Paint('6. PageDown')
        CheckHash(0x9E288BEF1DDF15B2)
        Key.Pressed(PageUp)
        Paint('7. PageUp')
        CheckHash(0xF99606F54DC84450)
        Key.Pressed(Ctrl+Down)
        Paint('8. Ctrl+Down')
        CheckHash(0x4FC99B4FC264B220)
        Key.Pressed(Ctrl+Up)
        Paint('9. Ctrl+Up')
        CheckHash(0xF99606F54DC84450)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_int_i8(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_int_i8_large_buffer_selection_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF99606F54DC84450)
        Key.Pressed(Shift+Right)
        Paint('2. Shift+Right')
        CheckHash(0x0D4D663F73B92988)
        Key.Pressed(Shift+Down)
        Paint('3. Shift+Down')
        CheckHash(0x7D0A746A8862A2BB)
        Key.Pressed(Shift+Left)
        Paint('4. Shift+Left')
        CheckHash(0xF4F9002BE8DB76D7)
        Key.Pressed(Shift+Up)
        Paint('5. Shift+Up')
        CheckHash(0xF99606F54DC84450)
        Key.Pressed(Shift+PageDown)
        Paint('6. Shift+PageDown')
        CheckHash(0x7725673268052EDE)
        Key.Pressed(Shift+PageUp)
        Paint('7. Shift+PageUp')
        CheckHash(0xF99606F54DC84450)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_int_i8(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_navigation_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Right,3)
        Paint('2. Move right 3 bytes')
        CheckHash(0xA7B831471BE92232)
        Key.Pressed(Down,2)
        Paint('3. Move down 2 rows')
        CheckHash(0x980A7B28D25C843E)
        Key.Pressed(Home)
        Paint('4. Home')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(End)
        Paint('5. End')
        CheckHash(0xDCC14E43416C4B62)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_selection_with_keys() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Shift+Right,4)
        Paint('2. Select 4 bytes')
        CheckHash(0x7535A9D8B019620E)
        Key.Pressed(Shift+Down)
        Paint('3. Extend selection down')
        CheckHash(0x6588F96C95851AEA)
        Key.Pressed(Home)
        Paint('4. Clear selection via navigation')
        CheckHash(0x6F386642B4BD3242)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_panel_toggle() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state on hex panel')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(End)
        Paint('2. Move to end')
        CheckHash(0xDCC14E43416C4B62)
        Key.Pressed(Ctrl+Tab)
        Paint('3. Switch to character panel')
        CheckHash(0x97F5B14CB49601B9)
        Key.Pressed(Ctrl+Tab)
        Paint('4. Switch back to hex panel')
        CheckHash(0xDCC14E43416C4B62)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_address_separator_resize() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Ctrl+Alt+Left)
        Paint('2. Address separator selected')
        CheckHash(0x7063C9C2CB8614BE)
        Key.Pressed(Right,2)
        Paint('3. Widen address column')
        CheckHash(0xA1F22FB44A9AAA9)
        Key.Pressed(Escape)
        Paint('4. Deselect separator')
        CheckHash(0xF15104EF97AE3D2D)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_address_column_widths_1_to_6() {
    let script = "
        Paint.Enable(false)
        Paint('1. Address column resize mode')
        Key.Pressed(Ctrl+Alt+Left)
        CheckHash(0x7063C9C2CB8614BE)
        Key.Pressed(Left,5)
        Paint('2. Address width 1')
        CheckHash(0xBA5C0A8634AF9970)
        Key.Pressed(Right)
        Paint('3. Address width 2')
        CheckHash(0xBEC97B4F8722E21A)
        Key.Pressed(Right)
        Paint('4. Address width 3')
        CheckHash(0xEA125174CADB42FA)
        Key.Pressed(Right)
        Paint('5. Address width 4')
        CheckHash(0x5B7593B2CF80649C)
        Key.Pressed(Right)
        Paint('6. Address width 5')
        CheckHash(0x67A69CE3345658CD)
        Key.Pressed(Right)
        Paint('7. Address width 6')
        CheckHash(0x7063C9C2CB8614BE)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_large_buffer_address_column_widths_1_to_6() {
    let script = "
        Paint.Enable(false)
        Paint('1. Address column resize mode')
        Key.Pressed(Ctrl+Alt+Left)
        CheckHash(0x3FCE180396C01AC8)
        Key.Pressed(Left,5)
        Paint('2. Address width 1')
        CheckHash(0x5C7293AE7C35B9C8)
        Key.Pressed(Right)
        Paint('3. Address width 2')
        CheckHash(0x7E3D71801237766C)
        Key.Pressed(Right)
        Paint('4. Address width 3')
        CheckHash(0x204DA662702AEB70)
        Key.Pressed(Right)
        Paint('5. Address width 4')
        CheckHash(0x71DF1C41AC295BB2)
        Key.Pressed(Right)
        Paint('6. Address width 5')
        CheckHash(0xAF26CCF49026B75F)
        Key.Pressed(Right)
        Paint('7. Address width 6')
        CheckHash(0x3FCE180396C01AC8)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_large_buffer_end_address_column_widths_1_to_6() {
    let script = "
        Paint.Enable(false)
        Key.Pressed(End)
        Paint('1. Cursor at end of large buffer')
        CheckHash(0x210BF035F3BCFF98)
        Paint('2. Address column resize mode')
        Key.Pressed(Ctrl+Alt+Left)
        CheckHash(0x47680DF6BA034F24)
        Key.Pressed(Left,5)
        Paint('3. Address width 1')
        CheckHash(0xD0771BA94A93460B)
        Key.Pressed(Right)
        Paint('4. Address width 2')
        CheckHash(0x662E1EA95FC946C9)
        Key.Pressed(Right)
        Paint('5. Address width 3')
        CheckHash(0xD4F0A515004AF0AA)
        Key.Pressed(Right)
        Paint('6. Address width 4')
        CheckHash(0x4453075645DB408)
        Key.Pressed(Right)
        Paint('7. Address width 5')
        CheckHash(0xA5278F728F229EA3)
        Key.Pressed(Right)
        Paint('8. Address width 6')
        CheckHash(0x47680DF6BA034F24)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_horizontal_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Ctrl+Right,3)
        Paint('2. Scroll view right')
        CheckHash(0x9DE18EF14198BEFA)
        Key.Pressed(Ctrl+Left,3)
        Paint('3. Scroll view back')
        CheckHash(0x6F386642B4BD3242)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

// Mouse coordinates in App::debug scripts are absolute desktop positions.
// For App::debug(60, 15) with window("Test,a:c,w:60,h:12"):
//   y=1 window title, y=2 column headers, y=3+ data rows
//   x=7 address separator, x=9 hex data, x=38 char data, x=45 char header

#[test]
fn check_bufferview_mouse_selection_hex_panel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Home)
        Mouse.Drag(9,3,21,3)
        Paint('2. Drag selection across hex panel')
        CheckHash(0x7535A9D8B019620E)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_click_hex_panel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial hex panel')
        CheckHash(0xF9BBE4D7A406E4C7)
        Mouse.Click(38,3,left)
        Paint('2. Char panel via data click')
        CheckHash(0xCE0B00E3E5EC6514)
        Mouse.Click(10,3,left)
        Paint('3. Hex panel via data click')
        CheckHash(0xF9BBE4D7A406E4C7)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_click_char_panel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial hex panel')
        CheckHash(0xF9BBE4D7A406E4C7)
        Mouse.Click(38,3,left)
        Paint('2. Click char panel')
        CheckHash(0xCE0B00E3E5EC6514)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_selection_char_panel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial')
        CheckHash(0x6F386642B4BD3242)
        Key.Pressed(Home)
        Mouse.Drag(38,3,48,3)
        Paint('2. Drag selection in char panel')
        CheckHash(0xF9918796DB5FB399)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_resize_address_column() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial')
        CheckHash(0xF9BBE4D7A406E4C7)
        Mouse.Drag(7,3,12,3)
        Paint('2. Wider address column')
        CheckHash(0x7D07C76FC76E53E4)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_resize_interval_column() {
    let script = "
        Paint.Enable(false)
        Paint('1. With intervals')
        CheckHash(0x633434DA55BE779A)
        Mouse.Move(19,3)
        Paint('2. Hover over interval column')
        CheckHash(0x7EAB4BD247931F62)
        Mouse.Drag(19,3,25,3)
        Paint('3. Wider interval column')
        CheckHash(0xD0678206131347B6)
    ";
    let mut a = App::debug(80, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:12,flags: Sizeable");
    w.add(make_bufferview_with_intervals(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_header_switch_panel() {
    let script = "
        Paint.Enable(false)
        Paint('1. Hex panel active')
        CheckHash(0xF9BBE4D7A406E4C7)
        Mouse.Click(45,2,left)
        Mouse.Click(38,3,left)
        Paint('2. Char panel via header')
        CheckHash(0xCE0B00E3E5EC6514)
        Mouse.Click(12,2,left)
        Mouse.Click(10,3,left)
        Paint('3. Hex panel via header')
        CheckHash(0xF9BBE4D7A406E4C7)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_wheel_vertical_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Top of large buffer')
        CheckHash(0x04ED34329C80A874)
        Mouse.Wheel(30,4,down,2)
        Paint('2. Scrolled down')
        CheckHash(0x267D1EB007F7B5CC)
        Mouse.Wheel(30,4,up,2)
        Paint('3. Scrolled back up')
        CheckHash(0x04ED34329C80A874)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_mouse_wheel_horizontal_scroll() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial view')
        CheckHash(0xC8FA57F31BCEBD5A)
        Mouse.Wheel(20,4,right,2)
        Paint('2. Scrolled right')
        CheckHash(0x23DA88FCE0286327)
        Mouse.Wheel(20,4,left,2)
        Paint('3. Scrolled back left')
        CheckHash(0xC8FA57F31BCEBD5A)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:40,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

// For App::debug(60, 15) with window("Test,a:c,w:40,h:12"):
//   y=1 window title, y=2 column headers, y=3-11 data rows, y=12 horizontal scrollbar
//   x=49 vertical scrollbar, x=15-28 horizontal scrollbar track on row 12

#[test]
fn check_bufferview_mouse_drag_scrollbars() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial view')
        CheckHash(0xCF6A659FFD85E28D)
        Mouse.Drag(49,3,49,9)
        Paint('2. Vertical scrollbar dragged')
        CheckHash(0x9794D33B0FAA9712)
        Mouse.Drag(15,12,28,12)
        Paint('3. Horizontal scrollbar dragged')
        CheckHash(0xBFDD15A7C38EAF0D)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:40,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_search() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Key.TypeText('hex:05')
        Paint('2. Search for 0x05')
        CheckHash(0x7FB0D059F32AD815)
        Key.Pressed(Enter)
        Paint('3. Find next match')
        CheckHash(0x6A4FB0875345E87E)
        Key.Pressed(Escape)
        Paint('4. Clear search')
        CheckHash(0x97CFA8E09EF9879D)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_search_bar_mouse_hex() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x04ED34329C80A874)
        Mouse.Click(3,12,left)
        Key.TypeText('hex:05')
        Paint('2. Search for 0x05')
        CheckHash(0x2646A1F4F66D9A47)
        Key.Pressed(Enter)
        Paint('3. Find next 0x05 match')
        CheckHash(0x1298E3D2B27D3E74)
        Key.Pressed(Escape)
        Paint('4. Clear search')
        CheckHash(0x062E51AD63B789E)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_search_bar_mouse_text() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Mouse.Click(3,12,left)
        Key.TypeText('text:Hello')
        Paint('2. Search for Hello')
        CheckHash(0xCA141C0D46AD4D79)
        Key.Pressed(Enter)
        Paint('3. Find next Hello match')
        CheckHash(0x37A4B939ABC9114B)
        Key.Pressed(Escape)
        Paint('4. Clear search')
        CheckHash(0x3116CE71CD75C896)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_search_bar_backspace_then_enter() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Mouse.Click(3,12,left)
        Key.TypeText('text:Hello')
        Paint('2. Search for Hello')
        CheckHash(0xCA141C0D46AD4D79)
        Key.Pressed(Backspace,2)
        Paint('3. Backspace twice')
        CheckHash(0x7E4B024AD0043F16)
        Key.Pressed(Enter)
        Paint('4. Find next match')
        CheckHash(0x5984B53AE5B978F8)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_search_without_search_bar_ignored() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0xF9BBE4D7A406E4C7)
        Mouse.Click(3,12,left)
        Key.TypeText('text:Hello')
        Paint('2. Search ignored without search bar')
        CheckHash(0x32434095719B8557)
        Key.Pressed(Enter)
        Paint('3. Enter ignored without search bar')
        CheckHash(0x32434095719B8557)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_for_mouse(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_oct_auto_columns_resize_window() {
    let script = "
        Paint.Enable(false)
        Paint('1. Narrow window')
        CheckHash(0x9A28583258DF4ED8)
        Mouse.Hold(84,12,left)
        Mouse.Move(119,12)
        Paint('2. Wider window')
        CheckHash(0xDE4C17B58AD74773)
        Mouse.Release(119,12,left)
    ";
    let mut a = App::debug(120, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    w.add(make_bufferview_oct_auto(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_auto_columns_resize_window() {
    let script = "
        Paint.Enable(false)
        Paint('1. Narrow window')
        CheckHash(0xE3C2615AE67D5849)
        Mouse.Hold(84,12,left)
        Mouse.Move(119,12)
        Paint('2. Wider window')
        CheckHash(0x0826A0DB4C1E085D)
        Mouse.Release(119,12,left)
    ";
    let mut a = App::debug(120, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    w.add(make_bufferview_char_auto(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_auto_columns_resize_window_large_buffer() {
    let script = "
        Paint.Enable(false)
        Paint('1. Narrow window')
        CheckHash(0x40D8324ED5567A4B)
        Mouse.Hold(84,12,left)
        Mouse.Move(119,12)
        Paint('2. Wider window')
        CheckHash(0x7326734E0EB4959D)
        Mouse.Release(119,12,left)
    ";
    let mut a = App::debug(120, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:50,h:12,flags: Sizeable");
    w.add(make_bufferview_char_auto(test_large_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_resize_window() {
    let script = "
        Paint.Enable(false)
        Paint('1. Initial state')
        CheckHash(0x6F386642B4BD3242)
        Resize(50,10)
        Paint('2. Smaller window')
        CheckHash(0x4E726C2EA666518F)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_interval_names() {
    let script = "
        Paint.Enable(false)
        Paint('1. With interval names')
        CheckHash(0x153E93FE5224A01A)
        Key.Pressed(Down,4)
        Paint('2. Cursor in body interval')
        CheckHash(0x1790DAF1F71A76BA)
        Key.Pressed(Ctrl+Alt+Left)
        Key.Pressed(Tab)
        Paint('3. Interval separator selected')
        CheckHash(0x3313D640B7233296)
        Key.Pressed(Right,3)
        Paint('4. Widen interval column')
        CheckHash(0xACAB1828B1E631BB)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:12,flags: Sizeable");
    w.add(make_bufferview_with_intervals(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_mode_navigation() {
    let script = "
        Paint.Enable(false)
        Paint('1. Character mode initial')
        CheckHash(0x88532F2DCC1AD765)
        Key.Pressed(Right,5)
        Paint('2. Move right 5 characters')
        CheckHash(0x360EC5EBED1B9F31)
        Key.Pressed(Down)
        Paint('3. Move down one row')
        CheckHash(0x3AF4D6D579EA196D)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_char_mode(test_buffer_data()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_interval_name_title() {
    let script = "
        Paint.Enable(false)
        Paint('Custom interval title')
        CheckHash(0xD10E808E690B2C7E)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:12,flags: Sizeable");
    let mut bv = make_bufferview_with_intervals(test_buffer_data());
    bv.set_interval_name_title("Section");
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_interval_name_width() {
    let script = "
        Paint.Enable(false)
        Paint('Wide interval column')
        CheckHash(0x8330455F2F84A636)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:70,h:12,flags: Sizeable");
    let mut bv = make_bufferview_with_intervals(test_buffer_data());
    bv.set_interval_name_width(12);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_ascii_strings_visible() {
    let script = "
        Paint.Enable(false)
        Paint('ASCII strings on')
        CheckHash(0x91B452D970FDCC4A)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview_char_mode(test_buffer_ascii_strings());
    bv.set_ascii_strings_visible(true);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_utf16_ascii_strings_visible() {
    let script = "
        Paint.Enable(false)
        Paint('UTF-16 ASCII strings on')
        CheckHash(0x88DDC3F309F563D2)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview_char_mode(test_buffer_utf16_ascii_strings());
    bv.set_utf16_ascii_strings_visible(true);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_decode_utf8_enabled() {
    let script = "
        Paint.Enable(false)
        Paint('UTF-8 decode on')
        CheckHash(0x67A496C8F49A9A1)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    let mut bv = make_bufferview_char_mode(test_buffer_utf8_chars());
    bv.set_decode_utf8(true);
    w.add(bv);
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_decode_utf8_disabled() {
    let script = "
        Paint.Enable(false)
        Paint('UTF-8 decode off')
        CheckHash(0x23D42635D30312EE)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_char_mode(test_buffer_utf8_chars()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_mode_all_decodings() {
    let script = "
        Paint.Enable(false)
        Paint('1. All char decodings enabled')
        CheckHash(0x9EEA6610A78D3C90)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_char_mode_all_decodings(test_buffer_char_mode_mixed()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_mode_all_decodings_utf8_section() {
    let script = "
        Paint.Enable(false)
        Paint('1. Top of mixed buffer')
        CheckHash(0x9EEA6610A78D3C90)
        Key.Pressed(Right,35)
        Paint('2. UTF-8 section in view')
        CheckHash(0xB81E6F94013F482C)
    ";
    let mut a = App::debug(60, 15, script).build().unwrap();
    let mut w = window!("Test,a:c,w:60,h:12,flags: Sizeable");
    w.add(make_bufferview_char_mode_all_decodings(test_buffer_char_mode_mixed()));
    a.add_window(w);
    a.run();
}

#[test]
fn check_bufferview_char_mode_feature_toggles_with_commands() {
    let script = "
        Paint.Enable(false)
        Paint('1. All features off')
        CheckHash(0x9077A224ABA7974E)
        Key.Pressed(F1)
        Paint('2. Address visible')
        CheckHash(0xACA7A6D7BEA85544)
        Key.Pressed(F2)
        Paint('3. Address hidden')
        CheckHash(0x9077A224ABA7974E)
        Key.Pressed(F3)
        Paint('4. Interval names visible')
        CheckHash(0xBDDBB7B2E8804D88)
        Key.Pressed(F4)
        Paint('5. Interval names hidden')
        CheckHash(0x9077A224ABA7974E)
        Key.Pressed(F5)
        Paint('6. UTF-16 ASCII strings visible')
        CheckHash(0x7495A6A3A4C292F6)
        Key.Pressed(F6)
        Paint('7. UTF-16 ASCII strings hidden')
        CheckHash(0x9077A224ABA7974E)
        Key.Pressed(F7)
        Paint('8. UTF-8 decode on')
        CheckHash(0x62238A0B5BC623C4)
        Key.Pressed(F8)
        Paint('9. UTF-8 decode off')
        CheckHash(0x9077A224ABA7974E)
    ";

    #[Window(events=CommandBarEvents,commands:A+B+C+D+E+F+G+H, internal:true)]
    struct MyWin {
        buffer_handle: Handle<BufferView<Vec<u8>>>,
    }

    impl MyWin {
        fn new() -> Self {
            let mut w = Self {
                base: window!("Test,a:c,w:70,h:14,flags: Sizeable"),
                buffer_handle: Handle::None,
            };
            w.buffer_handle = w.add(make_bufferview_char_mode_features(test_buffer_char_mode_large()));
            w
        }

        fn set_address_visible(&mut self, visible: bool) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                bv.set_address_visible(visible);
            }
        }

        fn set_interval_names_visible(&mut self, visible: bool) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                bv.set_interval_names_visible(visible);
            }
        }

        fn set_utf16_ascii_strings_visible(&mut self, visible: bool) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                bv.set_utf16_ascii_strings_visible(visible);
            }
        }

        fn set_decode_utf8(&mut self, enabled: bool) {
            let h = self.buffer_handle;
            if let Some(bv) = self.control_mut(h) {
                bv.set_decode_utf8(enabled);
            }
        }
    }

    impl CommandBarEvents for MyWin {
        fn on_update_commandbar(&self, commandbar: &mut CommandBar) {
            commandbar.set(key!("F1"), "Address On", mywin::Commands::A);
            commandbar.set(key!("F2"), "Address Off", mywin::Commands::B);
            commandbar.set(key!("F3"), "Intervals On", mywin::Commands::C);
            commandbar.set(key!("F4"), "Intervals Off", mywin::Commands::D);
            commandbar.set(key!("F5"), "Unicode On", mywin::Commands::E);
            commandbar.set(key!("F6"), "Unicode Off", mywin::Commands::F);
            commandbar.set(key!("F7"), "UTF-8 On", mywin::Commands::G);
            commandbar.set(key!("F8"), "UTF-8 Off", mywin::Commands::H);
        }

        fn on_event(&mut self, command_id: mywin::Commands) {
            match command_id {
                mywin::Commands::A => self.set_address_visible(true),
                mywin::Commands::B => self.set_address_visible(false),
                mywin::Commands::C => self.set_interval_names_visible(true),
                mywin::Commands::D => self.set_interval_names_visible(false),
                mywin::Commands::E => self.set_utf16_ascii_strings_visible(true),
                mywin::Commands::F => self.set_utf16_ascii_strings_visible(false),
                mywin::Commands::G => self.set_decode_utf8(true),
                mywin::Commands::H => self.set_decode_utf8(false),
            }
        }
    }

    let mut a = App::debug(80, 18, script).command_bar().build().unwrap();
    a.add_window(MyWin::new());
    a.run();
}
