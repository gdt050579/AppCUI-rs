pub fn string_to_bool(text: &str) -> Option<bool> {
    match text {
        "true" | "yes" => {
            return Some(true);
        }
        "false" | "no" => {
            return Some(false);
        }
        _ => {
            return None;
        }
    }
}
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
