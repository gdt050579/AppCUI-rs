#[inline]
pub(crate)  fn is_name_char(value: u8) -> bool {
    match value {
        b'a'..=b'z' => true,
        b'A'..=b'Z' => true,
        b'0'..=b'9' => true,
        b'_' => true,
        _ => false
    }
}

pub fn extract_structure_name(code: &str) -> String {
    if let Some(mut pos) = code.find("struct") {
        pos += 6;
        let buf = code.as_bytes();
        let len = buf.len();
        while (pos < len) && ((buf[pos] == b' ') || (buf[pos] == b'\t')) {
            pos += 1;
        }
        let start = pos;
        while (pos < len) && (is_name_char(buf[pos])) {
            pos += 1;
        }
        String::from(&code[start..pos])
    } else {
        panic!("Expecting a structure definition !");
    }
}
