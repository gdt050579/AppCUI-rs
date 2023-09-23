static LOWER_CASE_TABLE: [u8; 256] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
    39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 97, 98, 99, 100, 101, 102, 103, 104, 105,
    106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133,
    134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191,
    192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
    221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255,
];
pub(super) fn compute_hash(text: &str) -> u64 {
    let buf = text.as_bytes();
    // use FNV algorithm ==> https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
    if buf.len() == 0 {
        return 0;
    }
    let mut hash = 0xcbf29ce484222325u64;
    let mut idx = 0usize;
    while idx < buf.len() {
        hash = hash ^ (LOWER_CASE_TABLE[buf[idx] as usize] as u64);
        hash = hash.wrapping_mul(0x00000100000001B3u64);
        idx += 1;
    }
    return hash;
}
pub(super) fn equal_ignore_case(text1: &str, text2: &str) -> bool {
    if text1.len() != text2.len() {
        return false;
    }
    let b1 = text1.as_bytes();
    let b2 = text2.as_bytes();
    let len = b1.len();
    for i in 0..len {
        if LOWER_CASE_TABLE[b1[i] as usize] != LOWER_CASE_TABLE[b2[i] as usize] {
            return false;
        }
    }
    return true;
}
pub(super) fn to_i32(text: &str) -> Option<i32> {
    if let Ok(value) = text.parse::<i32>() {
        return Some(value);
    }
    None
}
pub(super) fn to_percentage(text: &str) -> Option<f32> {
    let buf = text.as_bytes();
    let len = buf.len();
    if len == 0 {
        return None;
    }
    if buf[len - 1] != b'%' {
        return None;
    }
    let mut first_part = 0i32;
    let mut second_part = 0i32;
    let mut negative = false;
    let mut pos = 0;
    let end = len - 1;

    if (end > 0) && (buf[0] == b'-') {
        negative = true;
        pos += 1;
    }
    while (pos < end) && ((buf[pos] >= b'0') && (buf[pos] <= b'9')) {
        first_part = first_part * 10 + ((buf[pos] - b'0') as i32);
        pos += 1;
    }
    if (pos < end) && (buf[pos] == b'.') {
        let mut cnt = 0;
        while (pos < end) && ((buf[pos] >= b'0') && (buf[pos] <= b'9')) {
            if cnt < 2 {
                second_part = second_part * 10 + ((buf[pos] - b'0') as i32);
                cnt += 1;
            }
            pos += 1;
        }
    }
    if pos != end {
        return None;
    }
    let mut fvalue = (first_part as f32) + (second_part as f32) / 100.0f32;
    if negative {
        fvalue = -fvalue;
    }
    return Some(fvalue);
}