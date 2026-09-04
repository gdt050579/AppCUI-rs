#[derive(Debug, PartialEq, Eq)]
pub(super) enum Error {
    EmptySearch,
    InvalidHex,
    InvalidU8,
    InvalidI8,
    InvalidU16,
    InvalidI16,
    InvalidU32,
    InvalidI32,
    InvalidU64,
    InvalidI64,
    InvalidF32,
    InvalidF64,
    UnclosedQuote,
    InvalidText,
}

#[inline(always)]
fn is_list_separator(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte == b','
}

#[inline(always)]
fn is_ascii_whitespace_byte(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r')
}

fn quoted_content(text: &str) -> Result<&str, Error> {
    let bytes = text.as_bytes();
    let quote = match bytes.first() {
        Some(b'"' | b'\'') => bytes[0],
        _ => return Err(Error::InvalidText),
    };
    for (index, &byte) in bytes[1..].iter().enumerate() {
        if byte == quote {
            if !bytes[1 + index + 1..]
                .iter()
                .all(|&byte| is_ascii_whitespace_byte(byte))
            {
                return Err(Error::InvalidText);
            }
            return Ok(&text[1..1 + index]);
        }
    }
    Err(Error::UnclosedQuote)
}

fn parse_quoted_text(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    output.extend_from_slice(quoted_content(text)?.as_bytes());
    Ok(())
}

fn parse_string_content(text: &str) -> Result<&str, Error> {
    let text = text.trim_start();
    if text.is_empty() {
        return Err(Error::EmptySearch);
    }
    match text.as_bytes().first() {
        Some(b'"' | b'\'') => quoted_content(text),
        _ => Ok(text),
    }
}

#[inline(always)]
fn push_utf16_le(ch: char, output: &mut Vec<u8>) {
    let code = ch as u32;
    if code <= 0xFFFF {
        output.extend_from_slice(&(code as u16).to_le_bytes());
        return;
    }
    let code = code - 0x10000;
    let high = 0xD800 + (code >> 10) as u16;
    let low = 0xDC00 + (code & 0x3FF) as u16;
    output.extend_from_slice(&high.to_le_bytes());
    output.extend_from_slice(&low.to_le_bytes());
}

fn parse_utf16(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    let content = parse_string_content(text)?;
    if content.is_empty() {
        return Err(Error::EmptySearch);
    }
    for ch in content.chars() {
        push_utf16_le(ch, output);
    }
    Ok(())
}

fn parse_text_bytes(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    match text.as_bytes().first() {
        Some(b'"' | b'\'') => parse_quoted_text(text, output),
        _ => {
            output.extend_from_slice(text.as_bytes());
            Ok(())
        }
    }
}

fn parse_hex(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    let mut high: Option<u8> = None;
    let mut had_digit = false;

    for &byte in text.as_bytes() {
        if is_list_separator(byte) {
            continue;
        }
        let nibble = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err(Error::InvalidHex),
        };
        had_digit = true;
        match high {
            None => high = Some(nibble),
            Some(h) => {
                output.push((h << 4) | nibble);
                high = None;
            }
        }
    }
    if !had_digit {
        return Err(Error::EmptySearch);
    }
    if high.is_some() {
        return Err(Error::InvalidHex);
    }
    Ok(())
}

macro_rules! create_parse_numeric_list {
    ($fn_name:ident, $ty:ty, $error:expr) => {
        fn $fn_name(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
            let mut had_value = false;
            for segment in text.split(|ch: char| ch.is_ascii_whitespace() || ch == ',') {
                if segment.is_empty() {
                    continue;
                }
                let value: $ty = segment.parse().map_err(|_| $error)?;
                output.extend_from_slice(&value.to_le_bytes());
                had_value = true;
            }
            if !had_value {
                return Err(Error::EmptySearch);
            }
            Ok(())
        }
    };
}

create_parse_numeric_list!(parse_u8, u8, Error::InvalidU8);
create_parse_numeric_list!(parse_i8, i8, Error::InvalidI8);
create_parse_numeric_list!(parse_u16, u16, Error::InvalidU16);
create_parse_numeric_list!(parse_i16, i16, Error::InvalidI16);
create_parse_numeric_list!(parse_u32, u32, Error::InvalidU32);
create_parse_numeric_list!(parse_i32, i32, Error::InvalidI32);
create_parse_numeric_list!(parse_u64, u64, Error::InvalidU64);
create_parse_numeric_list!(parse_i64, i64, Error::InvalidI64);
create_parse_numeric_list!(parse_f32, f32, Error::InvalidF32);
create_parse_numeric_list!(parse_f64, f64, Error::InvalidF64);

fn parse_text(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    parse_text_bytes(text.trim_start(), output)
}

pub(super) fn parse(text: &str, output: &mut Vec<u8>) -> Result<(), Error> {
    output.clear();
    let to_search = text.trim();
    if to_search.is_empty() {
        return Err(Error::EmptySearch);
    }
    if let Some(p) = to_search.strip_prefix("hex:") {
        return parse_hex(p, output);
    }
    if let Some(p) = to_search.strip_prefix("u8:") {
        return parse_u8(p, output);
    }
    if let Some(p) = to_search.strip_prefix("u16:") {
        return parse_u16(p, output);
    }
    if let Some(p) = to_search.strip_prefix("u32:") {
        return parse_u32(p, output);
    }
    if let Some(p) = to_search.strip_prefix("u64:") {
        return parse_u64(p, output);
    }
    if let Some(p) = to_search.strip_prefix("i8:") {
        return parse_i8(p, output);
    }
    if let Some(p) = to_search.strip_prefix("i16:") {
        return parse_i16(p, output);
    }
    if let Some(p) = to_search.strip_prefix("i32:") {
        return parse_i32(p, output);
    }
    if let Some(p) = to_search.strip_prefix("i64:") {
        return parse_i64(p, output);
    }
    if let Some(p) = to_search.strip_prefix("f32:") {
        return parse_f32(p, output);
    }
    if let Some(p) = to_search.strip_prefix("f64:") {
        return parse_f64(p, output);
    }
    if let Some(p) = to_search.strip_prefix("utf16:") {
        return parse_utf16(p, output);
    }
    if let Some(p) = to_search.strip_prefix("text:") {
        return parse_text(p, output);
    }

    // if no prefix, assume its a regular text and parse it as it is
    parse_text(to_search.trim(), output)
}
