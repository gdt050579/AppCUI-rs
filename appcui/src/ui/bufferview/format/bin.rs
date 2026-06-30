use super::super::OutputBuffer;

pub(super) fn write(bytes: [u8; 8], output: &mut OutputBuffer) {
    let v = bytes[0];
    for i in 0..8 {
        let bit = (v >> (7 - i)) & 1;
        output.set(i, if bit != 0 { b'1' } else { b'0' });
    }
    output.set_len(8);
}
