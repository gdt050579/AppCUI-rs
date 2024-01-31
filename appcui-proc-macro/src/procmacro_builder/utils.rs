pub fn validate_struct_name(name: &str) -> bool {
    if name.len() == 0 {
        return false;
    }
    for (index, ch) in name.char_indices() {
        if ((ch >= 'A') && (ch <= 'Z')) || ((ch >= 'a') && (ch <= 'z')) {
            continue;
        }
        if (ch >= '0') && (ch <= '9') {
            if index == 0 {
                return false;
            } else {
                continue;
            }
        }
        if ch == '_' {
            continue;
        }
        // else --> invalid character --> exit
        return false;
    }
    return true;
}
#[inline]
pub fn is_name_char(value: u8) -> bool {
    match value {
        b'a'..=b'z' => return true,
        b'A'..=b'Z' => return true,
        b'0'..=b'9' => return true,
        b'_' => return true,
        _ => return false
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
        return String::from(&code[start..pos]);
    } else {
        panic!("Expecting a structure definition !");
    }
}
